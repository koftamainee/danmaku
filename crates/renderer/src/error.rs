use thiserror::Error;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("Failed to create surface")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),

    #[error("Failed to request physical device")]
    RequestAdapter(#[from] wgpu::RequestAdapterError),
    
    #[error("Failed to request graphics device")]
    RequestDevice(#[from] wgpu::RequestDeviceError),

    #[error("Frame skipped, try again next frame")]
    FrameSkipped,

    #[error("Surface needs to be reconfigured")]
    SurfaceOutdated,

    #[error("Surface validation error")]
    ValidationError,
}
