# Renderer Audit Notes

Current renderer risks and optimization targets for the next pass:

- Per-mesh uniform and GPU buffers are created every frame. This is simple, but it is a CPU/GPU allocation hotspot. Move mesh data to persistent GPU resources and update only changed uniforms.
- Tilemap rendering emits one draw command per painted tile. This is acceptable for small maps, but large maps need chunked tile batching or cached instance buffers per layer.
- Background pass creates a uniform buffer every frame. Replace with a persistent buffer updated via `queue.write_buffer`.
- Mesh lighting is forward-only and sends the full point light array per mesh. This is fine for the MVP, but clustered/tiled light assignment or per-frame light buffers will scale better.
- Render sorting currently mixes 2D and 3D with simple order/depth rules. This is practical, but transparent 3D, particle systems, and UI-world overlays will need explicit render layers.
- World transform matrices are computed recursively during render. Cache dirty world matrices in `World` once hierarchy editing and runtime parenting become more common.
- Sprite and tile paths are still unlit. Decide deliberately whether 2D should stay unlit by default or receive a separate 2D lighting model.
- Several renderer match arms intentionally ignore UI/debug fields and currently produce warnings. Clean these up before enforcing warning-free CI.
