use crate::pipeline::Pipeline;
use crate::bind_group::BindGroup;
use crate::buffer::Buffer;
use crate::texture::Rect;

pub struct RenderPass<'a> {
    pub(crate) render_pass: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
    pub fn set_pipeline(&mut self, pipeline: &Pipeline) {
        self.render_pass.set_pipeline(&pipeline.wgpu_pipeline);
    }

    pub fn set_bind_group(&mut self, index: u32, group: &BindGroup) {
        self.render_pass.set_bind_group(index, &group.wgpu_bind_group, &[]);
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer) {
        self.render_pass.set_vertex_buffer(slot, buffer.wgpu_buffer.slice(..));
    }

    pub fn set_index_buffer(&mut self, buffer: &Buffer, format: wgpu::IndexFormat) {
        self.render_pass.set_index_buffer(buffer.wgpu_buffer.slice(..), format);
    }

    pub fn draw_indexed(&mut self, index_count: u32, instance_count: u32) {
        self.render_pass.draw_indexed(0..index_count, 0, 0..instance_count);
    }

    pub fn draw(&mut self, vertex_count: u32, instance_count: u32) {
        self.render_pass.draw(0..vertex_count, 0..instance_count);
    }

    pub fn set_viewport(&mut self, rect: &Rect) {
        self.render_pass.set_viewport(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            0.0,
            1.0,
        );
    }

    pub fn finish(self) {}
}
