// esbuild stubs for oracle-rendering examples that pull app-side components
// with heavy dependency trees (Streamdown, motion, base-nova selects).
// The stubs keep the DOM shape examples render with — Arabic-default
// translations, plain children pass-through — without the runtime deps.
import * as React from "react"

// language-selector: no provider → local default "ar"; the docs HOST page
// owns the language switcher (our preview-frame AR/HE/EN/FA buttons), so
// the select chrome is deliberately not rendered.
export function useTranslation(translations, defaultLanguage = "ar") {
  const { dir, locale, values: t } = translations[defaultLanguage] ?? {}
  return { language: defaultLanguage, setLanguage: () => {}, dir, locale, t }
}
export function LanguageProvider({ children, ...props }) {
  return React.createElement(React.Fragment, null, children)
}
export function LanguageSelector({ children, ...props }) {
  return React.createElement(React.Fragment, null, children)
}
export const languageOptions = []

// markdown: Streamdown wrapper — render children (demos pass markdown text
// or rich children) inside the slot div so data-slot structure is kept.
export function Markdown({ className, children, ...props }) {
  return React.createElement(
    "div",
    { "data-slot": "markdown", className, ...props },
    children
  )
}
