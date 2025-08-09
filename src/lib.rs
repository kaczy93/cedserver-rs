#![feature(test)]
extern crate test;

pub mod map;
pub mod server;
mod buf_utf8;
mod chunk_cache;
mod enums;
mod statics_chunk;
mod connection_handler;
mod land_chunk;
mod land_tile;
mod map_handlers;
mod mul_index;
mod net_state;
mod point;
mod static_tile;
mod benchmark_sandbox;
