document.addEventListener('DOMContentLoaded', () => {
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
