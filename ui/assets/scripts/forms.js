const INPUT_NUMBER_STEP = 5;

document.addEventListener('DOMContentLoaded', () => {
    // form buttons handler:
    document.querySelector('#main .blocks').addEventListener('click', (event) => {
        const target = event.target;

        if (target.closest('.input-plus')) {
            let parent = target.parentElement.parentElement.parentElement.querySelector('input[type="number"]');
            parent.value = Math.min(Math.max(+parent.value + INPUT_NUMBER_STEP, 0), 999);
            parent.focus();
        }
        if (target.closest('.input-minus')) {
            let parent = target.parentElement.parentElement.parentElement.querySelector('input[type="number"]');
            parent.value = Math.min(Math.max(+parent.value - INPUT_NUMBER_STEP, 0), 999);
            parent.focus();
        }
    });
});
