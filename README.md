# Hucky

Hucky - вебплатформа для програмувальних турнірів, хакатонів і командних IT-змагань. Вона покриває повний цикл події: реєстрацію користувачів, створення турніру, формування команд, інвайти, раунди, дедлайни, здачу робіт, оцінювання журі, leaderboard і сертифікати.

## Коротко про реалізацію

- серверна частина написана на Rust з Axum і SQLx
- інтерфейс зроблений на SvelteKit, Svelte 5, Vite і Tailwind CSS
- дані зберігаються в PostgreSQL, для локального старту є Docker Compose
- авторизація побудована на JWT, паролі хешуються через Argon2
- для перевірки є backend-тести на Cargo і frontend-тести на Vitest

## Маршрут по README

1. Функціональність Hucky
2. Як влаштована система
3. Мапа репозиторію
4. Стек і ключові залежності
5. Що треба встановити
6. Запуск на локальній машині
7. Сценарії для різних ролей
8. Команди для розробки
9. Конфігурація через env
10. Базовий production-сценарій
11. Підтримка після запуску
12. Нотатки з безпеки
13. Типові проблеми
14. Короткі відповіді

## 1. Функціональність Hucky

- реєстрація, login і JWT-сесія
- ролі: `admin`, `organiser`, `jury`, `participant`
- створення турнірів з правилами, датами, лімітом команд і статусами
- раунди з описом завдання, вимогами, стартом і дедлайном
- команди всередині конкретного турніру
- email-інвайти в команду, прийняття або відхилення запрошень
- здача GitHub URL, video demo URL, optional live demo URL і опису роботи
- призначення робіт журі з балансуванням навантаження
- оцінки за критеріями, коментарі журі та автоматичне завершення оцінювання
- leaderboard на основі реальних оцінок з бази даних
- генерація сертифікатів для учасників завершених турнірів
- backend automation для оновлення статусів турнірів, раундів і інвайтів

## 2. Як влаштована система

Frontend і backend запускаються як окремі сервіси. SvelteKit frontend працює на `http://localhost:5173`, а Rust backend слухає `http://127.0.0.1:8080`.

Ключові принципи:

- backend має API-префікс `/api`
- PostgreSQL є основним джерелом істини
- SQLx використовується для міграцій і типізованої роботи з БД
- frontend звертається до backend через server-side API helper
- бізнес-правила дублюються на рівні сервісів і частково через database constraints
- статуси турнірів і раундів можуть оновлюватися автоматично

Швидка перевірка API після запуску:

```powershell
curl http://127.0.0.1:8080/api/ping
```

## 3. Мапа репозиторію

```text
Hucky/
  backend/
    migrations/
    src/
      users/
      tournaments/
      rounds/
      teams/
      submissions/
      jury/
      evaluations/
      leaderboard/
      certificates/
    tests/
    Cargo.toml
    docker-compose.yml
    .env.example
  frontend/
    src/
      components/
      lib/
      routes/
    static/
    package.json
  README.md
```

Практично:

- `backend` відповідає за доменну логіку, авторизацію, API, міграції та PostgreSQL
- `frontend` відповідає за UI, сторінки, форми, PDF-сертифікати й роботу з API

## 4. Стек і ключові залежності

Backend:

- Rust 2024 edition
- Axum 0.7
- Tokio
- SQLx 0.8
- PostgreSQL
- JWT через `jsonwebtoken`
- Argon2 для хешування паролів
- Tower HTTP CORS

Frontend:

- SvelteKit 2
- Svelte 5
- Vite 7
- Tailwind CSS 4
- Vitest
- Testing Library for Svelte

Інфраструктура:

- Docker Compose для локальної PostgreSQL
- SQLx migrations
- npm для frontend-залежностей
- Cargo для backend-збірки й тестів

## 5. Що треба встановити

Обов'язково:

- Rust toolchain
- Cargo
- Node.js 20+
- npm
- Docker Desktop або інший Docker runtime
- SQLx CLI

Перевірка:

```powershell
rustc --version
cargo --version
node -v
npm -v
docker --version
sqlx --version
```

Якщо `sqlx` не встановлений:

```powershell
cargo install sqlx-cli --no-default-features --features postgres
```

## 6. Запуск на локальній машині

### 6.1. Перший вхід у проєкт

```powershell
git clone <repo-url>
cd Hucky
```

Frontend-залежності:

```powershell
cd frontend
npm install
```

Backend-залежності Cargo завантажить автоматично під час `cargo run`, `cargo build` або `cargo test`.

### 6.2. Конфіг backend

```powershell
cd backend
Copy-Item .env.example .env
```

Мінімальний локальний приклад:

```env
DATABASE_URL=postgres://admin:qwerty@localhost:5433/tournament_db
JWT_SECRET=replace-with-at-least-32-random-characters
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
FRONTEND_ORIGIN=http://localhost:5173
SUPERADMIN_EMAIL=admin@example.com
```

`JWT_SECRET` має бути не коротшим за 32 символи.

### 6.3. PostgreSQL і міграції

У папці `backend` підніміть PostgreSQL:

```powershell
docker compose up -d
```

Застосуйте міграції:

```powershell
sqlx migrate run
```

За замовчуванням база доступна на `localhost:5433`, pgAdmin - на `http://localhost:5050`.

### 6.4. Старт API

Термінал 1:

```powershell
cd backend
cargo run
```

Backend буде доступний тут:

```text
http://127.0.0.1:8080
```

### 6.5. Старт вебінтерфейсу

Термінал 2:

```powershell
cd frontend
npm run dev
```

Якщо Windows PowerShell блокує `npm`, використайте:

```powershell
npm.cmd run dev
```

Frontend буде доступний тут:

```text
http://localhost:5173
```

### 6.6. Smoke-check

```powershell
curl http://127.0.0.1:8080/api/ping
```

Очікувано API повертає JSON зі статусом успішної відповіді.

## 7. Сценарії для різних ролей

`admin`:

- переглядає користувачів
- змінює ролі
- не може випадково прибрати власну admin-роль

`organiser`:

- створює турніри
- додає раунди, дедлайни, вимоги та критерії
- керує статусами турніру
- переглядає команди
- додає журі
- закриває сабміти й генерує assignments

`jury`:

- бачить тільки призначені йому роботи
- відкриває GitHub, video demo та live demo посилання
- виставляє оцінки за критеріями
- залишає коментар

`participant`:

- реєструється та входить у систему
- створює команду в межах турніру
- приймає або відхиляє інвайти
- здає роботу від імені accepted-команди
- переглядає результати й сертифікати

Типовий життєвий цикл:

1. Організатор створює турнір і перший раунд.
2. Учасники створюють команди й приймають інвайти.
3. Турнір переходить у `running`.
4. Команди здають роботи в активний раунд.
5. Організатор генерує призначення журі.
6. Журі оцінює роботи.
7. Система рахує leaderboard.
8. Після завершення турніру учасники отримують сертифікати.

## 8. Команди для розробки

Backend:

```powershell
cd backend
cargo run          # dev запуск API
cargo build        # збірка backend
cargo test         # backend тести
sqlx migrate run   # застосувати міграції
docker compose up -d
docker compose down
```

Frontend:

```powershell
cd frontend
npm run dev            # dev сервер
npm run build          # production build
npm run preview        # preview production build
npm run test           # Vitest
npm run test:watch     # watch mode
npm run test:coverage  # coverage
npm run lint           # prettier check + eslint
npm run format         # prettier write
```

Для Windows PowerShell можна використовувати `npm.cmd` замість `npm`.

## 9. Конфігурація через env

Backend: `backend/.env`

Обов'язкові:

- `DATABASE_URL`
- `JWT_SECRET`

Опційні з дефолтами:

- `SERVER_HOST`, дефолт `127.0.0.1`
- `SERVER_PORT`, дефолт `8080`
- `FRONTEND_ORIGIN`, дефолт `http://localhost:5173`
- `SUPERADMIN_EMAIL`

Приклад:

```env
DATABASE_URL=postgres://admin:qwerty@localhost:5433/tournament_db
JWT_SECRET=replace-with-at-least-32-random-characters
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
FRONTEND_ORIGIN=http://localhost:5173
SUPERADMIN_EMAIL=admin@example.com
```

Frontend:

- `BACKEND_URL` можна задати для server-side запитів SvelteKit
- якщо змінна не задана, frontend використовує `http://127.0.0.1:8080`

Приклад локального запуску зі змінною:

```powershell
$env:BACKEND_URL="http://127.0.0.1:8080"
npm run dev
```

## 10. Базовий production-сценарій

Один з простих сценаріїв: VPS + PostgreSQL + systemd або PM2/process manager + Nginx reverse proxy.

### 10.1. Backend

```powershell
cd backend
Copy-Item .env.example .env
```

У production потрібно змінити:

```env
DATABASE_URL=postgres://user:strong_password@127.0.0.1:5432/hucky_db
JWT_SECRET=long-random-production-secret-minimum-32-characters
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
FRONTEND_ORIGIN=https://your-domain.com
SUPERADMIN_EMAIL=admin@your-domain.com
```

Далі:

```powershell
sqlx migrate run
cargo build --release
.\target\release\backend.exe
```

На Linux binary буде `./target/release/backend`.

### 10.2. Frontend

```powershell
cd frontend
npm ci
$env:BACKEND_URL="https://your-domain.com"
npm run build
npm run preview
```

Для реального production краще запускати SvelteKit через node adapter або процес-менеджер і проксувати домен через Nginx.

### 10.3. Nginx і HTTPS

Рекомендовано:

- проксувати `/api/` на backend
- проксувати `/` на frontend
- увімкнути HTTPS через Let's Encrypt
- не відкривати PostgreSQL назовні
- зберігати `.env` тільки на сервері

## 11. Підтримка після запуску

Корисні команди:

```powershell
docker compose ps
docker compose logs db
cargo test
npm.cmd run test
```

Бекап PostgreSQL:

```powershell
pg_dump "postgres://admin:qwerty@localhost:5433/tournament_db" > hucky_backup.sql
```

Оновлення локальної копії:

```powershell
git pull
cd backend
sqlx migrate run
cargo test
cd ../frontend
npm install
npm run build
```

## 12. Нотатки з безпеки

- не комітьте `.env`
- міняйте `JWT_SECRET` у production
- використовуйте секрет довжиною мінімум 32 символи
- не відкривайте PostgreSQL у public internet
- вмикайте HTTPS на production
- перевіряйте `FRONTEND_ORIGIN`, інакше браузер може блокувати CORS-запити
- не призначайте ролі вручну напряму в БД без потреби
- робіть backup перед production-міграціями

## 13. Типові проблеми

Backend не підключається до БД:

- перевірте `DATABASE_URL`
- переконайтеся, що `docker compose up -d` виконано в папці `backend`
- перевірте порт `5433`
- застосуйте `sqlx migrate run`

Помилка `JWT_SECRET must be at least 32 characters long`:

- задайте довший `JWT_SECRET` у `backend/.env`

CORS помилка в браузері:

- перевірте `FRONTEND_ORIGIN=http://localhost:5173`
- перезапустіть backend після зміни `.env`

Frontend не бачить API:

- перевірте, що backend працює на `http://127.0.0.1:8080`
- за потреби задайте `BACKEND_URL`
- перевірте `/api/ping`

`npm` не запускається в PowerShell:

- використайте `npm.cmd run dev` або `npm.cmd run test`

`sqlx` не знайдено:

```powershell
cargo install sqlx-cli --no-default-features --features postgres
```

## 14. Короткі відповіді

Питання: Чому backend на `8080`, а frontend на `5173`?  
Відповідь: `8080` заданий у backend `.env`, а `5173` є стандартним dev-портом Vite.

Питання: Чи можна запустити frontend без backend?  
Відповідь: UI відкриється, але сторінки, які потребують API, не зможуть отримати дані.

Питання: Де Swagger?  
Відповідь: У поточній версії Swagger не підключений. Для smoke-check використовуйте `GET /api/ping`.

Питання: Як створити admin-користувача?  
Відповідь: задайте `SUPERADMIN_EMAIL` у `backend/.env`, зареєструйте користувача з цим email, і система призначить йому роль `admin`.

Питання: Чи потрібна мінімальна кількість учасників у команді для сабміту?  
Відповідь: так, команда має мати мінімум двох accepted-учасників.
