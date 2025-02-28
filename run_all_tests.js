async function runKernel(kernel_url) {
    const num_iters = 4000;
    for (let i = 0; i <= num_iters; i++) {
        const adapter = await navigator.gpu.requestAdapter({ powerPreference : "high-performance" });
        const device = await adapter.requestDevice();
        const pipeline = device.createComputePipeline({
            layout: 'auto',
            compute: {
                module: device.createShaderModule({ code: await (await fetch('buggy.wgsl')).text() }),
                entryPoint: 'main'
            }
        });
        const buffer = device.createBuffer({
            size: Uint32Array.BYTES_PER_ELEMENT,
            usage: GPUBufferUsage.STORAGE
        });
        const bindGroup = device.createBindGroup({
            layout: pipeline.getBindGroupLayout(0),
            entries: [{ binding: 0, resource: { buffer: buffer, offset: 0, size: Uint32Array.BYTES_PER_ELEMENT }}]
        });

        const commandEncoder = device.createCommandEncoder();
        const computePass = commandEncoder.beginComputePass();
        computePass.setPipeline(pipeline);
        computePass.setBindGroup(0, bindGroup);
        computePass.dispatchWorkgroups(2);
        computePass.end();

        device.queue.submit([commandEncoder.finish()]);
    }
}
