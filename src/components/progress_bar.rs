// src/progress_bar.rs

use bevy::prelude::*;
use bevy::asset::{Assets, Handle};
use bevy::color::LinearRgba;
use bevy::ecs::prelude::{Component, Query, ResMut};
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, Shader};
use bevy::ui::{node_bundles::MaterialNodeBundle, Style, UiMaterial};
use bevy::utils::default;


/*
    This component should handle create data required to control a progress bar and connect with the shader code for progress bar.

    Notes:
    Had to use LinearRGB since I use Color the compiler will complain about a trait not satisfied with AsBindGroup.
*/


// Define a unique handle for the shader
pub const PROGRESS_BAR_HANDLE: Handle<Shader> = Handle::weak_from_u128(8714649747086695632918559878778085427);

/// The ProgressBar component.
#[derive(Component, Clone)]
pub struct ProgressBar {
    progress: f32, // A value between 0.0 and 1.0 representing the progress
    fill_color: LinearRgba,
    empty_color: LinearRgba,
}

impl ProgressBar {
    pub fn new(fill_color: LinearRgba, empty_color: LinearRgba) -> Self {
        Self {
            progress: 0.0,
            fill_color,
            empty_color,
        }
    }

    pub fn set_progress(&mut self, amount: f32) -> &mut Self {
        self.progress = amount.clamp(0.0, 1.0);
        self
    }

    pub fn get_progress(&self) -> f32 {
        self.progress
    }

    pub fn increase_progress(&mut self, amount: f32) -> &mut Self {
        self.progress = (self.progress + amount).clamp(0.0, 1.0);
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.progress = 0.0;
        self
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            progress: 0.0,
            fill_color: LinearRgba::rgb(1.0, 0.0, 0.0), // Default fill color red
            empty_color: LinearRgba::rgb(0.0, 0.0, 0.0), // Default empty color black
        }
    }
}

#[derive(Bundle)]
pub struct ProgressBarBundle {
    pub progressbar: ProgressBar,
    pub material_node_bundle: MaterialNodeBundle<ProgressBarMaterial>,
}

impl ProgressBarBundle {
    pub fn new(progressbar: ProgressBar, materials: &mut ResMut<Assets<ProgressBarMaterial>>) -> ProgressBarBundle {
        ProgressBarBundle {
            progressbar,
            material_node_bundle: MaterialNodeBundle {
                style: Style {
                    width: bevy::ui::Val::Percent(100.0),
                    ..default()
                },
                material: materials.add(ProgressBarMaterial::default()),
                ..default()
            },
        }
    }
}

/// The Material for the ProgressBar
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    empty_color: LinearRgba,
    #[uniform(1)]
    progress: f32,
    #[uniform(2)]
    fill_color: LinearRgba,
}

impl Default for ProgressBarMaterial {
    fn default() -> Self {
        Self {
            empty_color: LinearRgba::rgb(0.0, 0.0, 0.0),
            progress: 0.0,
            fill_color: LinearRgba::rgb(1.0, 0.0, 0.0),
        }
    }
}

impl ProgressBarMaterial {
    pub fn update(&mut self, bar: &ProgressBar) {
        self.empty_color = bar.empty_color;
        self.progress = bar.progress;
        self.fill_color = bar.fill_color;
    }
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/progress_bar.wgsl".into()
    }
}

/// Function to update the progress bar material based on the ProgressBar component's state
pub fn update_progress_bar(
    mut query: Query<(&ProgressBar, &Handle<ProgressBarMaterial>)>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    for (bar, handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(handle) {
            material.update(bar);
        }
    }
}
