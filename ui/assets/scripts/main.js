const invoke = window.__TAURI__.core.invoke;  // DO NOT REMOVE!!
const timers = new Map();

document.addEventListener('DOMContentLoaded', () => {
    // init bot profiles:
    invoke("get_bots", {})
        .then(profiles => {
            let blocks = document.querySelector('#main .blocks');
            
            profiles.forEach(profile => {
                blocks.insertAdjacentHTML('beforeend', profile);
            });
        })
        .catch(e => console.error(e));

    // update bot limits percentage:
    setInterval(() => {
        document.querySelectorAll('#main .blocks .block.active').forEach((block) => {
            let id = block.getAttribute('target');

            invoke("update_bot_limits", { id })
                .then(percents => {
                    block.querySelector('.progress-bar .value').textContent = percents;
                })
                .catch(e => console.error(e));
        });
    }, 5000);

    // init buttons 'start all' & 'stop all':
    document.querySelector('#header button.start-all').addEventListener('click', (e) => {
        document.querySelectorAll('#main .blocks .block:not(.create-bot):not(.active)').forEach((block) => {
            block.querySelector('button.start-farm').click();
        });
    });
    document.querySelector('#header button.stop-all').addEventListener('click', (e) => {
        document.querySelectorAll('#main .blocks .block:not(.create-bot).active').forEach((block) => {
            block.querySelector('button.stop-farm').click();
        });
    });

    // form update handlers:
    document.querySelector('#main .blocks').addEventListener('input', (e) => {
        let input = e.target;
        let id = input.getAttribute('target');

        let block = document.querySelector(`.block[target="${id}"]`);
        let form = block.querySelector('form.options');

        // reset form timer:
        if (timers.has(id)) {
            clearTimeout(timers.get(id));
        }

        // start form timer:
        timers.set(id, setTimeout(async () => {
            // serializing form data:
            const data = {};
            form.querySelectorAll('input, select, textarea').forEach(input => {
                const name = input.name;
                if (!name) return;

                if (input.type === 'checkbox') {
                    data[name] = input.checked;
                } else if (input.type === 'number') {
                    data[name] = input.value ? Number(input.value) : null;
                } else {
                    data[name] = input.value;
                }
            });

            block.querySelector('.name').textContent = data.name;

            invoke('update_bot', { id, data: JSON.stringify(data) })
                .then(limits => {
                    timers.delete(id);
                })
                .catch(e => console.error(e));
        }, 1000));
    });

    // buttons handlers:
    document.querySelector('#main .blocks').addEventListener('click', (e) => {
        let target = e.target;

        // creating a new bot profile:
        if (target.closest('.create-bot')) {
            let button = target.closest('.create-bot');

            invoke("create_bot", {})
                .then(profile => {
                    let blocks = document.querySelector('#main .blocks');
                    blocks.insertAdjacentHTML('beforeend', profile);
                })
                .catch(e => console.error(e));
            
            button.removeAttribute('disabled');
        }

        // removing a bot profile:
        else if (target.closest('.delete-bot')) {
            let button = target.closest('.delete-bot');
            let id = button.getAttribute("target");
            let bot = document.querySelector(`.block[target="${id}"]`);

            button.setAttribute('disabled', '');
            bot.setAttribute('disabled', '');

            invoke("delete_bot", { id })
                .then(_ => {
                    bot.remove();
                })
                .catch(e => console.error(e));
        }

        // starting farming:
        else if (target.closest('.start-farm')) {
            let button = target.closest('.start-farm');
            let id = button.getAttribute("target");
            let block = document.querySelector(`.block[target="${id}"]`);

            block.setAttribute('disabled', '');
            
            invoke("start_bot", { id })
                .then(_ => {
                    block.querySelector('.progress-bar').setAttribute('completed', '0');
                    
                    block.removeAttribute('disabled', '');
                    block.classList.add("active");
                })
                .catch(e => console.error(e));
        }

        // stoping farming:
        else if (target.closest('.stop-farm')) {
            let button = target.closest('.stop-farm');
            let id = button.getAttribute("target");
            let bot = document.querySelector(`.block[target="${id}"]`);

            bot.setAttribute('disabled', '');

            invoke("stop_bot", { id })
                .then(_ => {
                    bot.classList.remove("active");
                    bot.removeAttribute('disabled', '');
                })
                .catch(e => console.error(e));
        }
    });
});
