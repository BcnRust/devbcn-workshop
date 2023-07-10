# Frontend

In this guide, we'll be using [Dioxus](https://dioxuslabs.com/) as the frontend for our project. Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. Heavily inspired by React, Dioxus allows you to build apps for the Web, Desktop, Mobile, and more. Its core implementation can run anywhere with no platform-dependent linking, which means it's not intrinsically linked to WebSys like many other Rust frontend toolkits. However, it's important to note that Dioxus hasn't reached a stable release yet, so some APIs, particularly for Desktop, may still be unstable.

As for styling our app, we'll be using [Tailwind CSS](https://tailwindcss.com/). Tailwind is a highly customizable, low-level CSS framework that gives you all of the building blocks you need to build bespoke designs without any annoying opinionated styles you have to fight to override. You can set it up in your project, build something with it in an online playground, and even learn more about it directly from the team on their channel. Tailwind also offers a set of beautiful UI components crafted by its creators to help you speed up your development process​.

This combination of tools will allow us to concentrate our energy on frontend development in Rust, rather than spending excessive time on styling our app.

In our guide, we'll be providing hints on how to use Tailwind classes with our Dioxus components. This way, you can focus on the logic of your components, while still being able to apply responsive, modern styles to them.
