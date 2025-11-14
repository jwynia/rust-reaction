# Tailwind CSS Patterns for AI-Generated Leptos Components

## Overview

This document provides Tailwind CSS patterns optimized for AI code generation with Leptos. These patterns are production-ready, accessible, and follow best practices.

## Core Principles

1. **Utility-first**: Use Tailwind utility classes instead of inline styles
2. **Responsive**: Include responsive variants where appropriate
3. **Accessible**: Follow WCAG guidelines for colors and interactions
4. **Consistent**: Use consistent spacing, colors, and sizing scales
5. **AI-friendly**: Patterns are clear, predictable, and easy to generate

---

## Layout Patterns

### Container

```rust
// Centered container with max width
view! {
    <div class="max-w-7xl mx-auto px-4">
        // Content
    </div>
}
```

**Variants:**
- `max-w-sm` (384px) - Small cards, modals
- `max-w-md` (448px) - Forms, narrow content
- `max-w-lg` (512px) - Articles, blog posts
- `max-w-2xl` (672px) - Standard content
- `max-w-4xl` (896px) - Wide content
- `max-w-6xl` (1152px) - Full width layouts
- `max-w-7xl` (1280px) - Maximum standard width

### Flexbox Layout

```rust
// Horizontal flex with gap
view! {
    <div class="flex gap-4 items-center justify-between">
        <div>"Left"</div>
        <div>"Right"</div>
    </div>
}

// Vertical stack
view! {
    <div class="flex flex-col gap-3">
        <div>"Top"</div>
        <div>"Bottom"</div>
    </div>
}

// Centered content
view! {
    <div class="flex items-center justify-center min-h-screen">
        <div>"Centered"</div>
    </div>
}
```

**Common flex utilities:**
- `flex` - Enable flexbox
- `flex-col` - Vertical direction
- `flex-row` - Horizontal direction (default)
- `gap-{n}` - Space between items (0-96, or px)
- `items-start|center|end|stretch` - Align items vertically
- `justify-start|center|end|between|around|evenly` - Align items horizontally
- `flex-wrap` - Allow wrapping

### Grid Layout

```rust
// 2-column grid
view! {
    <div class="grid grid-cols-2 gap-4">
        <div>"Column 1"</div>
        <div>"Column 2"</div>
    </div>
}

// Responsive grid (1 col mobile, 2 col tablet, 3 col desktop)
view! {
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div>"Item 1"</div>
        <div>"Item 2"</div>
        <div>"Item 3"</div>
    </div>
}
```

**Grid utilities:**
- `grid-cols-{n}` - Number of columns (1-12)
- `grid-rows-{n}` - Number of rows (1-6)
- `gap-{n}` - Gap between cells
- `col-span-{n}` - Column span (1-12)
- `row-span-{n}` - Row span (1-6)

---

## Component Patterns

### Buttons

```rust
// Primary button
view! {
    <button
        on:click=handler
        class="px-6 py-3 bg-blue-600 text-white font-medium rounded-lg
               hover:bg-blue-700 active:bg-blue-800
               focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
               transition-colors duration-200">
        "Click Me"
    </button>
}

// Secondary button
view! {
    <button
        on:click=handler
        class="px-6 py-3 bg-gray-200 text-gray-800 font-medium rounded-lg
               hover:bg-gray-300 active:bg-gray-400
               focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2
               transition-colors duration-200">
        "Cancel"
    </button>
}

// Danger button
view! {
    <button
        on:click=handler
        class="px-6 py-3 bg-red-600 text-white font-medium rounded-lg
               hover:bg-red-700 active:bg-red-800
               focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2
               transition-colors duration-200">
        "Delete"
    </button>
}

// Success button
view! {
    <button
        on:click=handler
        class="px-6 py-3 bg-green-600 text-white font-medium rounded-lg
               hover:bg-green-700 active:bg-green-800
               focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2
               transition-colors duration-200">
        "Confirm"
    </button>
}

// Outline button
view! {
    <button
        on:click=handler
        class="px-6 py-3 border-2 border-blue-600 text-blue-600 font-medium rounded-lg
               hover:bg-blue-50 active:bg-blue-100
               focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
               transition-colors duration-200">
        "Outline"
    </button>
}

// Disabled button
view! {
    <button
        disabled
        class="px-6 py-3 bg-gray-300 text-gray-500 font-medium rounded-lg
               cursor-not-allowed opacity-60">
        "Disabled"
    </button>
}
```

**Button sizes:**
- Small: `px-4 py-2 text-sm`
- Medium: `px-6 py-3 text-base`
- Large: `px-8 py-4 text-lg`

### Form Inputs

```rust
// Text input
view! {
    <input
        type="text"
        placeholder="Enter text..."
        class="w-full px-4 py-3 border border-gray-300 rounded-lg
               focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
               placeholder:text-gray-400
               transition-all duration-200"/>
}

// Textarea
view! {
    <textarea
        placeholder="Enter text..."
        rows="4"
        class="w-full px-4 py-3 border border-gray-300 rounded-lg
               focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
               placeholder:text-gray-400 resize-none
               transition-all duration-200"/>
}

// Select dropdown
view! {
    <select class="w-full px-4 py-3 border border-gray-300 rounded-lg
                   focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                   bg-white cursor-pointer
                   transition-all duration-200">
        <option>"Option 1"</option>
        <option>"Option 2"</option>
        <option>"Option 3"</option>
    </select>
}

// Checkbox
view! {
    <label class="flex items-center gap-3 cursor-pointer">
        <input
            type="checkbox"
            class="w-5 h-5 text-blue-600 border-gray-300 rounded
                   focus:ring-2 focus:ring-blue-500"/>
        <span class="text-gray-700">"Accept terms"</span>
    </label>
}

// Radio button
view! {
    <label class="flex items-center gap-3 cursor-pointer">
        <input
            type="radio"
            name="option"
            class="w-5 h-5 text-blue-600 border-gray-300
                   focus:ring-2 focus:ring-blue-500"/>
        <span class="text-gray-700">"Option 1"</span>
    </label>
}
```

### Cards

```rust
// Basic card
view! {
    <div class="bg-white rounded-lg shadow-md p-6">
        <h3 class="text-xl font-semibold text-gray-800 mb-2">
            "Card Title"
        </h3>
        <p class="text-gray-600">
            "Card content goes here."
        </p>
    </div>
}

// Card with header and footer
view! {
    <div class="bg-white rounded-lg shadow-md overflow-hidden">
        <div class="bg-gray-50 px-6 py-4 border-b border-gray-200">
            <h3 class="text-xl font-semibold text-gray-800">
                "Header"
            </h3>
        </div>
        <div class="p-6">
            <p class="text-gray-600">"Content"</p>
        </div>
        <div class="bg-gray-50 px-6 py-4 border-t border-gray-200">
            <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                "Action"
            </button>
        </div>
    </div>
}

// Hover card
view! {
    <div class="bg-white rounded-lg shadow-md p-6
                hover:shadow-xl hover:scale-105
                transition-all duration-300 cursor-pointer">
        <h3 class="text-xl font-semibold text-gray-800 mb-2">
            "Hover Me"
        </h3>
    </div>
}
```

### Alerts & Notifications

```rust
// Info alert
view! {
    <div class="bg-blue-50 border-l-4 border-blue-500 p-4 rounded">
        <div class="flex items-start gap-3">
            <div class="text-blue-600">ℹ️</div>
            <div>
                <h4 class="font-semibold text-blue-800">"Information"</h4>
                <p class="text-blue-700 text-sm mt-1">
                    "This is an informational message."
                </p>
            </div>
        </div>
    </div>
}

// Success alert
view! {
    <div class="bg-green-50 border-l-4 border-green-500 p-4 rounded">
        <div class="flex items-start gap-3">
            <div class="text-green-600">✓</div>
            <div>
                <h4 class="font-semibold text-green-800">"Success"</h4>
                <p class="text-green-700 text-sm mt-1">
                    "Operation completed successfully."
                </p>
            </div>
        </div>
    </div>
}

// Warning alert
view! {
    <div class="bg-yellow-50 border-l-4 border-yellow-500 p-4 rounded">
        <div class="flex items-start gap-3">
            <div class="text-yellow-600">⚠️</div>
            <div>
                <h4 class="font-semibold text-yellow-800">"Warning"</h4>
                <p class="text-yellow-700 text-sm mt-1">
                    "Please review this carefully."
                </p>
            </div>
        </div>
    </div>
}

// Error alert
view! {
    <div class="bg-red-50 border-l-4 border-red-500 p-4 rounded">
        <div class="flex items-start gap-3">
            <div class="text-red-600">✕</div>
            <div>
                <h4 class="font-semibold text-red-800">"Error"</h4>
                <p class="text-red-700 text-sm mt-1">
                    "Something went wrong."
                </p>
            </div>
        </div>
    </div>
}
```

### Badges & Tags

```rust
// Badge
view! {
    <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium
                 bg-blue-100 text-blue-800">
        "New"
    </span>
}

// Status badges
view! {
    <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium
                 bg-green-100 text-green-800">
        "Active"
    </span>
    <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium
                 bg-gray-100 text-gray-800">
        "Inactive"
    </span>
    <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium
                 bg-red-100 text-red-800">
        "Error"
    </span>
}
```

---

## Typography Patterns

### Headings

```rust
view! {
    <h1 class="text-4xl font-bold text-gray-900 mb-4">
        "Main Heading"
    </h1>
    <h2 class="text-3xl font-semibold text-gray-800 mb-3">
        "Section Heading"
    </h2>
    <h3 class="text-2xl font-semibold text-gray-800 mb-2">
        "Subsection Heading"
    </h3>
    <h4 class="text-xl font-medium text-gray-700 mb-2">
        "Minor Heading"
    </h4>
}
```

### Text

```rust
view! {
    // Body text
    <p class="text-base text-gray-600 leading-relaxed">
        "Regular paragraph text with comfortable line height."
    </p>

    // Small text
    <p class="text-sm text-gray-500">
        "Smaller helper text or captions."
    </p>

    // Large text
    <p class="text-lg text-gray-700">
        "Emphasized or important text."
    </p>

    // Muted text
    <p class="text-gray-400">
        "De-emphasized or disabled text."
    </p>
}
```

### Links

```rust
view! {
    <a href="#" class="text-blue-600 hover:text-blue-800 hover:underline">
        "Standard link"
    </a>
}
```

---

## Color Palette Reference

### Primary Colors (Blue)
- `bg-blue-50` to `bg-blue-950` - Background shades
- `text-blue-600` - Primary text
- `border-blue-500` - Primary borders
- `ring-blue-500` - Focus rings

### Semantic Colors

**Success (Green):**
- `bg-green-600`, `text-white` - Success buttons
- `bg-green-50`, `text-green-800` - Success alerts

**Warning (Yellow):**
- `bg-yellow-600`, `text-white` - Warning buttons
- `bg-yellow-50`, `text-yellow-800` - Warning alerts

**Danger (Red):**
- `bg-red-600`, `text-white` - Danger buttons
- `bg-red-50`, `text-red-800` - Error alerts

**Neutral (Gray):**
- `bg-gray-100` to `bg-gray-900` - Neutral backgrounds
- `text-gray-600` - Body text
- `text-gray-800` - Headings

---

## Spacing Scale

Use consistent spacing throughout:

- `gap-1` (0.25rem / 4px)
- `gap-2` (0.5rem / 8px)
- `gap-3` (0.75rem / 12px)
- `gap-4` (1rem / 16px)
- `gap-6` (1.5rem / 24px)
- `gap-8` (2rem / 32px)
- `gap-12` (3rem / 48px)

Same values apply to padding (`p-*`), margin (`m-*`), and other spacing utilities.

---

## Responsive Design

### Breakpoints

- `sm:` - 640px and up (tablets)
- `md:` - 768px and up (small laptops)
- `lg:` - 1024px and up (desktops)
- `xl:` - 1280px and up (large screens)
- `2xl:` - 1536px and up (extra large screens)

### Responsive Patterns

```rust
// Mobile-first responsive text
view! {
    <h1 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl">
        "Responsive Heading"
    </h1>
}

// Responsive grid
view! {
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        // Items
    </div>
}

// Hide on mobile, show on desktop
view! {
    <div class="hidden lg:block">
        "Desktop only"
    </div>
}

// Show on mobile, hide on desktop
view! {
    <div class="block lg:hidden">
        "Mobile only"
    </div>
}
```

---

## Common Combinations

### Form Group

```rust
view! {
    <div class="space-y-2">
        <label class="block text-sm font-medium text-gray-700">
            "Email Address"
        </label>
        <input
            type="email"
            placeholder="you@example.com"
            class="w-full px-4 py-3 border border-gray-300 rounded-lg
                   focus:ring-2 focus:ring-blue-500 focus:border-transparent"/>
        <p class="text-sm text-gray-500">
            "We'll never share your email."
        </p>
    </div>
}
```

### Modal/Dialog

```rust
view! {
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
        <div class="bg-white rounded-xl max-w-md w-full shadow-2xl">
            <div class="p-6 border-b border-gray-200">
                <h2 class="text-2xl font-semibold text-gray-800">
                    "Dialog Title"
                </h2>
            </div>
            <div class="p-6">
                <p class="text-gray-600">
                    "Dialog content goes here."
                </p>
            </div>
            <div class="p-6 border-t border-gray-200 flex gap-3 justify-end">
                <button class="px-6 py-3 bg-gray-200 text-gray-800 rounded-lg hover:bg-gray-300">
                    "Cancel"
                </button>
                <button class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                    "Confirm"
                </button>
            </div>
        </div>
    </div>
}
```

### Loading Spinner

```rust
view! {
    <div class="flex items-center justify-center p-8">
        <div class="w-12 h-12 border-4 border-gray-200 border-t-blue-600 rounded-full animate-spin"/>
    </div>
}
```

---

## Best Practices for AI Generation

1. **Always include hover states** for interactive elements
2. **Include focus states** for accessibility (keyboard navigation)
3. **Use semantic colors** (blue for primary, red for danger, etc.)
4. **Maintain consistent spacing** (use the scale: 2, 3, 4, 6, 8)
5. **Add transitions** for smooth interactions (`transition-colors duration-200`)
6. **Make it responsive** when appropriate
7. **Consider dark mode** if needed (use `dark:` variants)
8. **Use proper contrast** for accessibility (WCAG AA minimum)

---

## Quick Reference Card

**Most Common Classes:**
- Layout: `flex`, `grid`, `gap-4`, `p-6`, `mx-auto`, `max-w-2xl`
- Colors: `bg-blue-600`, `text-white`, `text-gray-600`, `border-gray-300`
- Sizing: `w-full`, `h-screen`, `text-base`, `px-6 py-3`
- Effects: `rounded-lg`, `shadow-md`, `hover:bg-blue-700`, `transition-colors`
- Typography: `font-medium`, `font-semibold`, `font-bold`, `text-sm|base|lg|xl`

**Button Template:**
```
px-6 py-3 bg-{color}-600 text-white rounded-lg hover:bg-{color}-700 transition-colors
```

**Input Template:**
```
w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500
```

**Card Template:**
```
bg-white rounded-lg shadow-md p-6
```

---

**Last Updated:** 2025-11-14
**For:** Leptos 0.6 + Tailwind CSS CDN
**Purpose:** AI-generated component styling reference
