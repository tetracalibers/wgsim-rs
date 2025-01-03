use std::error::Error;
use std::time::Duration;

use wgpu::util::DeviceExt;

use image::GenericImageView;

use wgsim::app::App;
use wgsim::ctx::DrawingContext;
use wgsim::ppl::RenderPipelineBuilder;
use wgsim::primitive::Size;
use wgsim::render::Render;
use wgsim::util;

const SAMPLER_BINDING_TYPE: wgpu::BindingType =
  wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering);

const TEXTURE_BINDING_TYPE: wgpu::BindingType = wgpu::BindingType::Texture {
  sample_type: wgpu::TextureSampleType::Float { filterable: true },
  view_dimension: wgpu::TextureViewDimension::D2,
  multisampled: false,
};

const UNIFORM_BINDING_TYPE: wgpu::BindingType = wgpu::BindingType::Buffer {
  ty: wgpu::BufferBindingType::Uniform,
  has_dynamic_offset: false,
  min_binding_size: None,
};

fn main() -> Result<(), Box<dyn Error>> {
  env_logger::init();

  let initial = setup();

  let mut app: App<State> = App::new("screen-fit-contain-image", initial);
  app.run()?;

  Ok(())
}

fn setup() -> Initial {
  let img_bytes = include_bytes!("../assets/img/pastel-tomixy.png");
  let image = image::load_from_memory(img_bytes).unwrap();
  let image_size = image.dimensions();

  Initial { image, image_size }
}

struct Initial {
  image: image::DynamicImage,
  image_size: (u32, u32),
}

struct State {
  render_result_bind_group: wgpu::BindGroup,
  render_result_pipeline: wgpu::RenderPipeline,

  resolution_uniform_buffer: wgpu::Buffer,
  need_resolution_update: bool,
}

impl<'a> Render<'a> for State {
  type Initial = Initial;

  async fn new(ctx: &DrawingContext<'a>, initial: &Self::Initial) -> Self {
    let render_shader =
      ctx.device.create_shader_module(wgpu::include_wgsl!("./render.wgsl"));

    let src_texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
      label: Some("src texture"),
      size: wgpu::Extent3d {
        width: initial.image_size.0,
        height: initial.image_size.1,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb, // 元画像の色味を保つため、ここだけsRGB
      usage: wgpu::TextureUsages::COPY_DST
        | wgpu::TextureUsages::RENDER_ATTACHMENT
        | wgpu::TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    });

    ctx.queue.write_texture(
      src_texture.as_image_copy(),
      &initial.image.to_rgba8(),
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(4 * initial.image_size.0),
        rows_per_image: Some(initial.image_size.1),
      },
      wgpu::Extent3d {
        width: initial.image_size.0,
        height: initial.image_size.1,
        depth_or_array_layers: 1,
      },
    );

    let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
      label: Some("sampler"),
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Linear,
      ..Default::default()
    });

    // リサイズのたびに更新する必要がある
    let resolution = ctx.resolution();
    let resolution_uniform_buffer =
      ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("resolution uniform buffer"),
        contents: bytemuck::cast_slice(&[
          resolution.width as f32, // シェーダ側での型に合わせるため、f32にキャスト
          resolution.height as f32,
        ]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      });

    let render_result_bind_group_layout = util::create_bind_group_layout(
      &ctx.device,
      &[
        SAMPLER_BINDING_TYPE,
        TEXTURE_BINDING_TYPE,
        UNIFORM_BINDING_TYPE,
      ],
      &[
        wgpu::ShaderStages::FRAGMENT,
        wgpu::ShaderStages::VERTEX_FRAGMENT,
        wgpu::ShaderStages::VERTEX,
      ],
    );

    let render_result_bind_group = util::create_bind_group(
      &ctx.device,
      &render_result_bind_group_layout,
      &[
        wgpu::BindingResource::Sampler(&sampler),
        wgpu::BindingResource::TextureView(
          &src_texture.create_view(&wgpu::TextureViewDescriptor::default()),
        ),
        resolution_uniform_buffer.as_entire_binding(),
      ],
    );

    let render_result_pipeline_layout =
      ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("render result pipeline layout"),
        bind_group_layouts: &[&render_result_bind_group_layout],
        push_constant_ranges: &[],
      });

    let render_result_pipeline = RenderPipelineBuilder::new(&ctx)
      .vs_shader(&render_shader, "vs_main")
      .fs_shader(&render_shader, "fs_main")
      .pipeline_layout(&render_result_pipeline_layout)
      .build();

    Self {
      render_result_bind_group,
      render_result_pipeline,

      resolution_uniform_buffer,
      need_resolution_update: false,
    }
  }

  fn resize(&mut self, ctx: &mut DrawingContext, size: Size) {
    if size.width > 0 && size.height > 0 {
      ctx.resize(size);
      self.need_resolution_update = true;
    }
  }

  fn update(&mut self, ctx: &DrawingContext, _dt: Duration) {
    if self.need_resolution_update {
      let resolution = ctx.resolution();
      ctx.queue.write_buffer(
        &self.resolution_uniform_buffer,
        0,
        bytemuck::cast_slice(&[
          resolution.width as f32,
          resolution.height as f32,
        ]),
      );
      self.need_resolution_update = false;
    }
  }

  fn draw(
    &mut self,
    encoder: &mut wgpu::CommandEncoder,
    render_target_view: &wgpu::TextureView,
    _sample_count: u32,
  ) -> Result<(), wgpu::SurfaceError> {
    let color_attachment = wgpu::RenderPassColorAttachment {
      view: render_target_view,
      resolve_target: None,
      ops: wgpu::Operations {
        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
        store: wgpu::StoreOp::Store,
      },
    };

    let mut render_pass =
      encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[Some(color_attachment)],
        ..Default::default()
      });

    render_pass.set_pipeline(&self.render_result_pipeline);
    render_pass.set_bind_group(0, &self.render_result_bind_group, &[]);
    render_pass.draw(0..6, 0..1);

    drop(render_pass);

    Ok(())
  }
}
