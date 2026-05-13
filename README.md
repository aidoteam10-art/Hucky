# Hucky
Programming tournament system

## Tests

The project now has a basic safety net for both sides of the app.

Backend tests check the Rust business rules and a small API layer:

```powershell
cd backend
cargo test
```

Frontend tests check real Svelte components:

```powershell
cd frontend
npm.cmd run test
```

On Windows PowerShell, use `npm.cmd` if plain `npm` is blocked by the execution policy.

In simple words: backend tests make sure the tournament rules do not break, and frontend tests make sure important UI pieces still show the right information and react correctly.
