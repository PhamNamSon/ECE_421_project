// import * as wasm from "project-3";

document.addEventListener("DOMContentLoaded", function() {
    const colorModes = {
        normal: {
            background: '#0000FF',
            user: '#FF0000',
            computer: '#FFFF00'
        },
        colorBlind1: {
            background: '#87CEEB',
            user: '#FF00FF',
            computer: '#FFA500'
        },
        colorBlind2: {
            background: '#B0C4DE',
            user: '#008080',
            computer: '#FFC0CB'
        }
    };

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
        const isCustomSize = document.querySelector('input[name="boardSize"]:checked').value === "custom";
        const selectedColorMode = document.querySelector('input[name="colorMode"]:checked').value;
        const colors = colorModes[selectedColorMode];

        const cols = isCustomSize ? parseInt(document.getElementById('customCols').value, 10) : 7;
        const rows = isCustomSize ? parseInt(document.getElementById('customRows').value, 10) : 6;

        const padding = 10;
        const holeDiameter = 80;

        const totalWidth = cols * (holeDiameter + padding) + padding;
        const totalHeight = rows * (holeDiameter + padding) + padding;

        gameboard.width = totalWidth;
        gameboard.height = totalHeight;

        startGameButton.disabled = true;
        document.querySelectorAll('input, button').forEach(function(item) {
            item.disabled = true;
        });

        ctx.fillStyle = colors.background;
        ctx.fillRect(0, 0, gameboard.width, gameboard.height);

        const holeRadius = holeDiameter / 2;
        ctx.fillStyle = 'white';
        for (let row = 0; row < rows; row++) {
            for (let col = 0; col < cols; col++) {
                const centerX = col * (holeDiameter + padding) + holeRadius + padding;
                const centerY = row * (holeDiameter + padding) + holeRadius + padding;
                ctx.beginPath();
                ctx.arc(centerX, centerY, holeRadius, 0, 2 * Math.PI);
                ctx.fill();
            }
        }
    });
});
