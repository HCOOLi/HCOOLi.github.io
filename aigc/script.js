const canvas = document.getElementById('gameBoard');
const ctx = canvas.getContext('2d');

const boardSize = 4;
const cellSize = canvas.width / boardSize;
let draggingPiece = null;
let pieces = [];
let gameOver = false; // 游戏结束标志

// 初始化棋盘，白棋在上，黑棋在下
function initializeBoard() {
    // 白棋
    for (let i = 0; i < boardSize; i++) {
        pieces.push({
            x: i * cellSize,
            y: 0,
            color: 'white',
        });
    }
    // 黑棋
    for (let i = 0; i < boardSize; i++) {
        pieces.push({
            x: i * cellSize,
            y: 3 * cellSize,
            color: 'black',
        });
    }
}

// 绘制棋盘和棋子
function drawBoard() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawGrid();
    drawPieces();
    if (gameOver) {
        displayGameOverMessage();
    }
}

// 绘制网格
function drawGrid() {
    for (let i = 0; i <= boardSize; i++) {
        ctx.beginPath();
        ctx.moveTo(i * cellSize, 0);
        ctx.lineTo(i * cellSize, canvas.height);
        ctx.moveTo(0, i * cellSize);
        ctx.lineTo(canvas.width, i * cellSize);
        ctx.stroke();
    }
}

// 绘制棋子
function drawPieces() {
    pieces.forEach(piece => {
        ctx.beginPath();
        ctx.arc(piece.x + cellSize / 2, piece.y + cellSize / 2, cellSize / 3, 0, Math.PI * 2);
        ctx.fillStyle = piece.color;
        ctx.fill();
        ctx.stroke();
    });
}

// 显示游戏结束消息
function displayGameOverMessage() {
    ctx.font = "30px Arial";
    ctx.fillStyle = "red";
    ctx.textAlign = "center";
    ctx.fillText("Game Over!", canvas.width / 2, canvas.height / 2 - 20);

    const winner = countPieces('white') === 1 ? "Black Wins!" : "White Wins!";
    ctx.fillText(winner, canvas.width / 2, canvas.height / 2 + 20);
}

// 统计棋盘上指定颜色的棋子数量
function countPieces(color) {
    return pieces.filter(piece => piece.color === color).length;
}

// 检查鼠标点击是否在棋子上
function getPieceAt(x, y) {
    return pieces.find(piece => {
        return x >= piece.x && x <= piece.x + cellSize &&
            y >= piece.y && y <= piece.y + cellSize;
    });
}

// 拖动棋子
canvas.addEventListener('mousedown', (e) => {
    if (gameOver) return; // 游戏结束后停止棋子拖动
    const mouseX = e.offsetX;
    const mouseY = e.offsetY;
    draggingPiece = getPieceAt(mouseX, mouseY);
});

canvas.addEventListener('mousemove', (e) => {
    if (draggingPiece && !gameOver) {
        const mouseX = e.offsetX - cellSize / 2;
        const mouseY = e.offsetY - cellSize / 2;
        draggingPiece.x = Math.max(0, Math.min(mouseX, canvas.width - cellSize));
        draggingPiece.y = Math.max(0, Math.min(mouseY, canvas.height - cellSize));
        drawBoard();
    }
});

canvas.addEventListener('mouseup', () => {
    if (draggingPiece && !gameOver) {
        // 将棋子放到最近的网格位置
        draggingPiece.x = Math.floor(draggingPiece.x / cellSize) * cellSize;
        draggingPiece.y = Math.floor(draggingPiece.y / cellSize) * cellSize;
        checkCapture(draggingPiece);
        draggingPiece = null;
        drawBoard();
        checkGameOver(); // 移动后检查游戏是否结束
    }
});

// 检查是否形成特定的棋子排列 (黑-黑-白 或 白-白-黑)，并吃掉中间的对方棋子
function checkCapture(piece) {
    const color = piece.color;
    const opponentColor = color === 'white' ? 'black' : 'white';

    // 水平方向检测 (黑-黑-白 或 白-白-黑)
    pieces.forEach(otherPiece => {
        if (otherPiece.y === piece.y) {
            const distance = Math.abs(otherPiece.x - piece.x);
            if (distance === cellSize * 2) {
                const middleX = (otherPiece.x + piece.x) / 2;
                const middlePiece = getPieceAt(middleX, piece.y);
                if (middlePiece && middlePiece.color !== color && middlePiece.color === opponentColor) {
                    if (color === 'black' && middlePiece.color === 'white') {
                        console.log('黑黑白排列，白棋被吃掉！');
                        pieces.splice(pieces.indexOf(middlePiece), 1); // 吃掉中间的白棋
                    } else if (color === 'white' && middlePiece.color === 'black') {
                        console.log('白白黑排列，黑棋被吃掉！');
                        pieces.splice(pieces.indexOf(middlePiece), 1); // 吃掉中间的黑棋
                    }
                }
            }
        }
    });

    // 垂直方向检测 (黑-黑-白 或 白-白-黑)
    pieces.forEach(otherPiece => {
        if (otherPiece.x === piece.x) {
            const distance = Math.abs(otherPiece.y - piece.y);
            if (distance === cellSize * 2) {
                const middleY = (otherPiece.y + piece.y) / 2;
                const middlePiece = getPieceAt(piece.x, middleY);
                if (middlePiece && middlePiece.color !== color && middlePiece.color === opponentColor) {
                    if (color === 'black' && middlePiece.color === 'white') {
                        console.log('黑黑白排列，白棋被吃掉！');
                        pieces.splice(pieces.indexOf(middlePiece), 1); // 吃掉中间的白棋
                    } else if (color === 'white' && middlePiece.color === 'black') {
                        console.log('白白黑排列，黑棋被吃掉！');
                        pieces.splice(pieces.indexOf(middlePiece), 1); // 吃掉中间的黑棋
                    }
                }
            }
        }
    });
}

// 检查游戏是否结束
function checkGameOver() {
    const blackCount = countPieces('black');
    const whiteCount = countPieces('white');

    if (blackCount === 1 || whiteCount === 1) {
        gameOver = true;
        console.log(blackCount === 1 ? '黑棋输了！' : '白棋输了！');
    }
}

initializeBoard();
drawBoard();