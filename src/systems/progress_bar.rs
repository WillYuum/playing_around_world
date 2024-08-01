// src/progress_bar.rs

use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::asset::{load_internal_asset, prelude::Assets, Asset, Handle};
use bevy::color::{Color, LinearRgba};
use bevy::ecs::prelude::{Bundle, Component, Query, ResMut};
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, Shader};
use bevy::ui::{node_bundles::MaterialNodeBundle, Style, UiMaterial, UiMaterialPlugin};
use bevy::utils::default;

// Define a unique handle for the shader
pub const PROGRESS_BAR_HANDLE: Handle<Shader> = Handle::weak_from_u128(8714649747086695632918559878778085427);

/// The ProgressBar component.
#[derive(Component, Clone)]
pub struct ProgressBar {
    progress: f32, // A value between 0.0 and 1.0 representing the progress
    pub sections: Vec<(u32, LinearRgba)>, // Different sections of the progress bar
    pub empty_color: LinearRgba, // Color of the empty space
}

impl ProgressBar {
    pub fn new(sections: Vec<(u32, LinearRgba)>) -> Self {
        Self {
            progress: 0.0,
            sections,
            empty_color: LinearRgba::ZERO,
        }
    }

    pub fn single(color: LinearRgba) -> Self {
        Self {
            progress: 0.0,
            sections: vec![(1, color)],
            empty_color: LinearRgba::NONE,
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
        self.progress += amount;
        self.progress = self.progress.clamp(0.0, 1.0);
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.progress = 0.0;
        self
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn clear_sections(&mut self) -> &mut Self {
        self.sections.clear();
        self
    }

    pub fn add_section(&mut self, amount: u32, color: LinearRgba) -> &mut Self {
        self.sections.push((amount, color));
        self
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            progress: 0.0,
            sections: vec![],
            empty_color: LinearRgba::NONE,
        }
    }
}

#[derive(Bundle)]
pub struct ProgressBarBundle {
    progressbar: ProgressBar,
    material_node_bundle: MaterialNodeBundle<ProgressBarMaterial>,
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
    /// The color of each section
    #[storage(2, read_only)]
    sections_color: Vec<LinearRgba>,
    #[storage(3, read_only)]
    sections_start_percentage: Vec<f32>,
    /// the length of the `sections_color` / `sections_start_percentage` vec.
    /// needs to be set for the shader
    #[uniform(4)]
    sections_count: u32,
}

impl Default for ProgressBarMaterial {
    fn default() -> Self {
        Self {
            empty_color: LinearRgba::rgb(0.0, 0.0, 0.0),
            progress: 0.0,
            sections_color: vec![],
            sections_start_percentage: vec![],
            sections_count: 0,
        }
    }
}

impl ProgressBarMaterial {
    pub fn update(&mut self, bar: &ProgressBar) {
        self.empty_color = bar.empty_color;
        self.progress = bar.progress;
        self.sections_color.clear();
        self.sections_start_percentage.clear();

        let total_amount: u32 = bar.sections.iter().map(|(amount, _)| amount).sum();
        for (amount, color) in &bar.sections {
            self.sections_start_percentage
                .push(*amount as f32 / total_amount as f32);
            self.sections_color.push(*color);
        }
        self.sections_count = bar.sections.len() as u32;
    }
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        PROGRESS_BAR_HANDLE.into()
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