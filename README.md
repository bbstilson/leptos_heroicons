# Leptos Heroicons

[Heroicons](https://heroicons.com/) Component for [Leptos](https://leptos.dev/)

## Usage

For each icon you want to use, you can add a feature flag. Only the icons you specify will be compiled into your application.

```toml
leptos_heroicons = { version = "0.1.0", features = [
  "24-solid-power",
  "24-outline-wrench",
  "20-solid-credit-card"
]}
```

You can also include all the icons of a given type by specifying the `size-type`:

```toml
leptos_heroicons = { version = "0.1.0", features = ["24-solid"] }
```

In your leptos code:

```rs
use leptos::*;
use leptos_heroions::size_24::solid::Power;

#[component]
fn MyComponent(cx: Scope) -> impl IntoView {
    view! { cx,
        <Power />
    }
}
```

## Inspiration / Credit

- [Heroicons](https://heroicons.com/)
- [Relm4 Icons](https://github.com/Relm4/icons)
- [Yew Heroicons](https://git.sr.ht/~ankhers/yew_heroicons).
