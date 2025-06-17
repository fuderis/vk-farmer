const INPUT_NUMBER_STEP = 5;

document.addEventListener('DOMContentLoaded', () => {
    // form buttons handler:
    document.querySelector('#main .blocks').addEventListener('click', (event) => {
        const target = event.target;

        if (target.closest('.input-plus')) {
            let input = target.parentElement.parentElement.parentElement.querySelector('input[type="number"]');
            input.value = Math.min(Math.max(+input.value + INPUT_NUMBER_STEP, 0), 999);
            
            input.focus();
            input.dispatchEvent(new Event('input', { bubbles: true }));
        }
        else if (target.closest('.input-minus')) {
            let input = target.parentElement.parentElement.parentElement.querySelector('input[type="number"]');
            input.value = Math.min(Math.max(+input.value - INPUT_NUMBER_STEP, 0), 999);

            input.focus();
            input.dispatchEvent(new Event('input', { bubbles: true }));
        }
    });
    
    // rotating progress-bar gradient:
    const root = document.documentElement;
    let angle = 0;

    function rotate() {
        angle = (angle + 1) % 360;
        root.style.setProperty('--progress-bar-deg', angle + 'deg');
        requestAnimationFrame(rotate);
    }

    rotate();
});
