import init, {setup, edit_bitmap} from '/wasm/project.js';

const renderCanvas = (ctx, video, bitmapConversionCanvas) => {
    const bitmapconversionContext = bitmapConversionCanvas.getContext("2d");
    requestAnimationFrame(async () => {
        try {
            bitmapconversionContext.drawImage(video, 0, 0);
            const bitmap = bitmapconversionContext.getImageData(0, 0, 400, 400);
            edit_bitmap(bitmap.data, 400, 400);
            ctx.putImageData(bitmap, 0, 0);
        } catch(e){
            console.error(e);
        } finally { 
            renderCanvas(ctx, video, bitmapConversionCanvas, bitmapconversionContext);
        }
    });
};

const initCamera = async (video, canvas, bitmapConversionCanvas) => {
    const stream = await navigator.mediaDevices.getUserMedia({video: {width: 400, height: 400}});
    video.srcObject = stream;
    video.onloadedmetadata = () => {
        video.play();
    };
    canvas.width = 400;
    canvas.height = 400;
    renderCanvas(canvas.getContext("2d"), video, bitmapConversionCanvas);
}

(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    setup();
    const video = document.querySelector("video");
    const canvas = document.querySelector("canvas");
    const bitmapConversionCanvas = new OffscreenCanvas(400, 400);
    initCamera(video, canvas, bitmapConversionCanvas);
})();
