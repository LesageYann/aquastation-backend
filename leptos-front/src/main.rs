use leptos::*;
use leptos_router::*;
use crate::pages::{logging::LoggingPage, not_found::NotFound, user::{dashboard::DashboardPage, containers::{list::ContainersPage, create::CreateContainerPage, create_probes::CreateProbePage}}};

mod atoms;
mod models;
mod molecules;
mod pages;
mod app_contexts;
mod config;
mod api;

fn main() {
    mount_to_body(|| {
        view! {
        <Router>
            <Routes>
                <Route path="/" view=LoggingPage/>
                <Route path="/dashboard" view=DashboardPage/>
                <Route path="/containers/new"  view=CreateContainerPage/>
                <Route path="/containers/:id/probes/new"  view=CreateProbePage/>
                <Route path="/containers"  view=ContainersPage/>
                <Route path="/*any" view=NotFound/>
            </Routes>
        </Router>
        }
    })
}