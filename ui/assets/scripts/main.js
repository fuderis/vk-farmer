const tauri = window.__TAURI__;     // DO NOT REMOVE!!
const invoke = tauri.core.invoke;   // DO NOT REMOVE!!
const events = tauri.event;         // DO NOT REMOVE!!
const timers = new Map();


//            E V E N T S:

// update program logs:
events.listen('update-logs', ({ payload }) => {
    const { log } = payload;

    // add log line:
    let logger = document.querySelector('#logger');
    logger.insertAdjacentHTML('beforeend', `<div class="line">${log}</div>`);

    // scroll logs down:
    logger.scrollTop += logger.scrollHeight;
});

// update bot progress:
events.listen('update-bot-progress', ({ payload }) => {
    const { bot_id, progress } = payload;

    // update progress bar:
    let bot_block = document.querySelector(`#main .blocks .block[target="${bot_id}"]`);
    bot_block.querySelector('.progress-bar .value').textContent = progress;
});


//            H A N D L E R S:

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

    // init button 'start all':
    document.querySelector('#header button.start-all').addEventListener('click', (e) => {
        document.querySelectorAll('#main .blocks .block:not(.create-bot):not(.active)').forEach((block) => {
            block.querySelector('button.start-farm').click();
        });
    });
    // init button 'stop all':
    document.querySelector('#header button.stop-all').addEventListener('click', (e) => {
        document.querySelectorAll('#main .blocks .block:not(.create-bot).active').forEach((block) => {
            block.querySelector('button.stop-farm').click();
        });
    });

    // forms update handler:
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

    // button handlers:
    document.querySelector('#main .blocks').addEventListener('click', (e) => {
        let target = e.target;

        // create a new bot profile:
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

        // remove a bot profile:
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

        // start farming:
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

        // stop farming:
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
