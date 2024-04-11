document.addEventListener("DOMContentLoaded", function() {
    document.querySelectorAll('input[name="boardSize"]').forEach((input) => {
        input.addEventListener('change', function(event) {
            if (event.target.value === "custom") {
                document.getElementById('customSizeInputs').style.display = 'inline-flex';
            } else {
                document.getElementById('customSizeInputs').style.display = 'none';
            }
        });
    });

    const startGameButton = document.getElementById('startGameButton');
    const gameboard = document.getElementById('gameboard');
    const ctx = gameboard.getContext('2d');

    startGameButton.addEventListener('click', function() {
        startGameButton.disabled = true;

        document.querySelectorAll('input, button').forEach(function(item) {
            item.disabled = true;
        });

        ctx.fillStyle = 'blue';
        ctx.fillRect(0, 0, gameboard.width, gameboard.height);

        const cols = 7;
        const rows = 6;
        const padding = 10;
        const holeDiameter = Math.min(
            (gameboard.width - padding * (cols + 1)) / cols,
            (gameboard.height - padding * (rows + 1)) / rows
        );
        const holeRadius = holeDiameter / 2;

        ctx.fillStyle = 'white';
        for (let row = 0; row < rows; row++) {
            for (let col = 0; col < cols; col++) {
                const centerX = col * (holeDiameter + padding) + holeDiameter / 2 + padding;
                const centerY = row * (holeDiameter + padding) + holeDiameter / 2 + padding;
                ctx.beginPath();
                ctx.arc(centerX, centerY, holeRadius, 0, 2 * Math.PI);
                ctx.fill();
            }
        }
    });
});
