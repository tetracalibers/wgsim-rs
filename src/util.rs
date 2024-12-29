pub fn create_bind_group_layout(
  device: &wgpu::Device,
  binding_types: &[wgpu::BindingType],
  shader_stages: &[wgpu::ShaderStages],
) -> wgpu::BindGroupLayout {
  let entries = shader_stages.iter().enumerate().map(|(i, stage)| {
    wgpu::BindGroupLayoutEntry {
      binding: i as u32,
      visibility: *stage,
      ty: binding_types[i],
      count: None,
    }
  });

  device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some("[wgsim] bind group layout"),
    entries: entries.collect::<Vec<_>>().as_slice(),
  })
}

pub fn create_bind_group(
  device: &wgpu::Device,
  layout: &wgpu::BindGroupLayout,
  resources: &[wgpu::BindingResource],
) -> wgpu::BindGroup {
  let entries =
    resources.iter().enumerate().map(|(i, resource)| wgpu::BindGroupEntry {
      binding: i as u32,
      resource: resource.clone(),
    });

  device.create_bind_group(&wgpu::BindGroupDescriptor {
    label: Some("[wgsim] bind group"),
    layout: &layout,
    entries: &entries.collect::<Vec<_>>(),
  })
}
