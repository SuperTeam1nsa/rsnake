//! # Game Modules
//!
//! This module organizes various components of the game logic, including:
//!

//! - `logic`: handles high level game logic.
//! - `render`: render all the game elements.
//! - `thread_manager`: manage the game thread.
//! - `state`: maintains the game's state, including score and status.
//! - `fruits_manager`: handles fruit-related logic and management.

//!
//! These modules collectively contribute to the core mechanics of the game logic.
//!

pub mod fruits_manager;
pub mod playing_logic;
pub mod playing_thread_manager;
pub mod state;
