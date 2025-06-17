const invoke = window.__TAURI__.core.invoke;  // DO NOT REMOVE!!

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

    // form update handlers:
    const blocksContainer = document.querySelector('.blocks');
    const timers = new Map();

    blocksContainer.addEventListener('input', (event) => {
        const input = event.target;
        const block = input.closest('.block');
        if (!block) return;

        const form = block.querySelector('form');
        if (!form) return;

        const id = block.getAttribute('target');
        if (!id) return;

        // reset form timer:
        if (timers.has(id)) {
            clearTimeout(timers.get(id));
        }

        // start form timer:
        timers.set(id, setTimeout(async () => {
            // serializing form data:
            const formData = new FormData(form);
            const data = {};
            formData.forEach((value, key) => {
                data[key] = value;
            });

            invoke('update_bot', { id, data })
                .then(_ => {
                    timers.delete(id);
                })
                .catch(e => console.error(e));
        }, 3000));
    });

    // buttons handlers:
    document.querySelector('#main .blocks').addEventListener('click', (event) => {
        const target = event.target;

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
            let bot = document.querySelector(`.block[target="${id}"]`);

            button.setAttribute('disabled', '');
            
            invoke("start_bot", { id })
                .then(r => {})
                .catch(e => console.error(e));
            
            button.removeAttribute('disabled');
            bot.classList.add("active");
        }

        // stoping farming:
        else if (target.closest('.stop-farm')) {
            let button = target.closest('.stop-farm');
            let id = button.getAttribute("target");
            let bot = document.querySelector(`.block[target="${id}"]`);

            button.setAttribute('disabled', '');

            invoke("stop_bot", { id })
                .then(r => {})
                .catch(e => console.error(e));

            button.removeAttribute('disabled');
            bot.classList.remove("active");
        }
    });
});
