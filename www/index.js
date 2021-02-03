import {memory} from "../pkg/wasm_learn_bg.wasm";
import {init, Universe} from "../pkg/wasm_learn";

init()

const CELL_SIZE = 5;
const GRID_COLOR = "#CCC";
const DEAD_COLOR = "#FFF";
const ALIVE_COLOR = "#000";


const universe = Universe.new()
const width = universe.width()
const height = universe.height()

const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext("2d");

canvas.height = height * (CELL_SIZE + 1) + 1
canvas.width = width * (CELL_SIZE + 1) + 1

let animateId = null;
let frameTickNumber = 1;


const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps")
        this.frames = []
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        const now = performance.now()
        const delta = now - this.lastFrameTimeStamp
        this.lastFrameTimeStamp = now
        const fps = 1000 / delta;

        this.frames.push(fps)
        if (this.frames.length > 100) {
            this.frames.shift()
        }

        let min = Infinity
        let max = -Infinity
        let sum = 0
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i]
            min = Math.min(this.frames[i], min)
            max = Math.max(this.frames[i], max)
        }
        let mean = sum / this.frames.length
        this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
        `.trim()
    }
}

const render_loop = () => {

    for (let i = 0; i < frameTickNumber; i++) {
        fps.render()
        universe.trick()
        drawGrid()
        drawCells()
    }

    animateId = requestAnimationFrame(render_loop)
}

function drawGrid() {
    ctx.beginPath()
    ctx.strokeStyle = GRID_COLOR

    for (let i = 0; i <= width; i++) {
        ctx.moveTo((CELL_SIZE + 1) * i + 1, 0);
        ctx.lineTo((CELL_SIZE + 1) * i + 1, canvas.height)
    }

    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, (CELL_SIZE + 1) * j + 1)
        ctx.lineTo(canvas.width, (CELL_SIZE + 1) * j + 1)
    }

    ctx.stroke()
}

function drawCells() {
    // cells 方法暴露的指针为 u32 类型
    const cellsPtr = universe.cells()
    // 在内存优化后每一个状态表示仅占用1个byte
    // 0 0 0 0 0 0 0 0
    // 0 0 0 0 0 0 0 0
    // 0 0 0 0 0 0 0 0
    // 0 0 0 0 0 0 0 1
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8)
    // u32 -> u8[1,0,0,0]
    ctx.beginPath()

    function getIndex(row, col) {
        return row * width + col
    }

    function bitIsSet(idx, cells) {
        // 00000011 00000000 00000000 00000000
        // [3,0,0,0]
        const byte = Math.floor(idx / 8); // 所以任意8个位分组的下标均一致
        const mask = 1 << (idx % 8) // 当前所在二进制下标,00000001 << 0 ===> 00000001
        // 00000011 &
        // 00000001
        // 00000001 当下标为0时为 true
        return (cells[byte] & mask) === mask
    }

    // for (let row = 0; row < height; row++) {
    //     for (let col = 0; col < width; col++) {
    //
    //         const idx = getIndex(row, col)
    //
    //         ctx.fillStyle = bitIsSet(idx, cells)
    //             ? ALIVE_COLOR : DEAD_COLOR
    //
    //         ctx.fillRect(
    //             col * (CELL_SIZE + 1) + 1,
    //             row * (CELL_SIZE + 1) + 1,
    //             CELL_SIZE,
    //             CELL_SIZE,
    //         )
    //     }
    // }

    ctx.fillStyle = ALIVE_COLOR
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col)
            if(!bitIsSet(idx, cells)) {
                continue
            }
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE,
            )
        }
    }

    ctx.fillStyle = DEAD_COLOR
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col)
            if(bitIsSet(idx, cells)) {
                continue
            }
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE,
            )
        }
    }

    ctx.stroke()

}

function isPause() {
    return animateId === null
}

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "pause"
    render_loop()
}

const pause = () => {
    playPauseButton.textContent = "play"
    cancelAnimationFrame(animateId)
    animateId = null
}

playPauseButton.addEventListener("click", e => {
    if (isPause()) {
        play()
    } else {
        pause()
    }
})

canvas.addEventListener("click", e => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
    const canvasTop = (e.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    if (isCtrlDown) {
        const delta = [
            [0, 0],
            [1, 0],
            [2, 0],
            [0, 1],
            [1, 2]
        ]
        delta.forEach(([x, y]) => universe.dead_cell(row + y, col + x))
    } else {
        universe.toggle_cell(row, col);
    }

    drawGrid()
    drawCells()

})

const resetUniverse = document.getElementById("reset-universe");

resetUniverse.addEventListener("click", e => {
    universe.reset_universe();
})

const deadUniverse = document.getElementById("dead-universe");

deadUniverse.addEventListener("click", e => {
    universe.dead_universe()
})

const frameTick = document.getElementById("frame-tick");

frameTick.addEventListener("input", e => {
    const value = e.target.value
    frameTickNumber = Number(value)
})

let isCtrlDown = false

document.addEventListener("keydown", e => {
    isCtrlDown = true
})
document.addEventListener("keyup", e => {
    isCtrlDown = false
})


play()