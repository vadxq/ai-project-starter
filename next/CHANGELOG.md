# Changelog

## Unreleased

### Changed

- Rebase the application on upstream 81e089fbcc06a237353e2107bd578d20567dfba3: Next 16.4.2, React 19.3.1, Tailwind, shadcn/ui, next-themes, typed locale routing, and native local checks.
- Rebuild the sign-in and Item workspace using the upstream components and theme tokens; keep private requests in the browser and prepare standalone Node output.
- Adapt API connectors to code/message/data responses, upstream errors, and integer millisecond timestamps. Regenerate clients from the new OpenAPI snapshot.

### Removed

- Exclude the starter demo and PWA features from this adaptation.

Local formatting, lint, locale checks, type checks, production and isolated builds, standalone HTTP smoke, and five real browser tests passed. Production audit reports no vulnerabilities; inherited development tool advisories remain documented.
