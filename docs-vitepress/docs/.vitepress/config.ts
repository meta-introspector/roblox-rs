import { defineConfig } from 'vitepress'

// https://vitepress.vuejs.org/config/app-configs
export default defineConfig({
  title: 'roblox-rs',
  description: 'Modern Rust Development for Roblox Games',
  themeConfig: {
    sidebar: [
      {
        text: 'Tutorials',
        items: [
          { text: 'Getting Started', link: '/tutorials/getting-started' },
          { text: 'Hello World ECS', link: '/tutorials/hello-world-ecs' },
          { text: 'Simple GUI', link: '/tutorials/simple-gui' },
        ],
      },
      {
        text: 'How-to Guides',
        items: [
          { text: 'Compile Rust to Luau', link: '/how-to/compile-rust-to-luau' },
          { text: 'Create Reactive GUI', link: '/how-to/create-reactive-gui' },
          { text: 'Style GUI Components', link: '/how-to/style-gui-components' },
          { text: 'Integrate GUI with ECS', link: '/how-to/integrate-gui-with-ecs' },
          { text: 'Advanced State Management', link: '/how-to/advanced-state-management' },
          { text: 'Advanced Compiler Options', link: '/how-to/advanced-compiler-options' },
          { text: 'Optimize Performance', link: '/how-to/optimize-performance' },
          { text: 'Debugging Compiled Code', link: '/how-to/debugging-compiled-code' },
        ],
      },
      {
        text: 'Reference',
        items: [
          { text: 'roblox-rs-core', link: '/reference/reference-core' },
          { text: 'roblox-rs-ecs', link: '/reference/reference-ecs' },
          { text: 'roblox-rs-cli', link: '/reference/reference-cli' },
          { text: 'roblox-rs-gui', link: '/reference/reference-gui' },
        ],
      },
      {
        text: 'Explanation',
        items: [
          { text: 'Architecture', link: '/explanation/architecture' },
          { text: 'Rust-to-Luau Transformation', link: '/explanation/rust-to-luau' },
        ],
      },
    ],
  },
})
