# CampusConnect: Every club. Every event. One brutally simple OS.

![License](https://img.shields.io/github/license/krushit1307/CampusConnect?style=flat-square)
![Open Issues](https://img.shields.io/github/issues/krushit1307/CampusConnect?style=flat-square)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)
![Stars](https://img.shields.io/github/stars/krushit1307/CampusConnect?style=flat-square)
![ECSoC 2026](https://img.shields.io/badge/ECSoC-2026-blueviolet?style=flat-square)

CampusConnect solves the chaos of college clubs juggling WhatsApp groups, spreadsheets, and paper certificates. It provides a single, unified platform for students and organizers to manage events, track memberships, and engage with their campus community seamlessly.

<!-- TODO: Add a demo screenshot or Loom link here -->
<!-- ![CampusConnect Demo](./public/demo.gif) -->

## ✨ Features

- **Event Management:** Create, manage, and promote campus events.
- **RSVP + QR Check-in:** Seamless registration and fast, verifiable QR code check-ins.
- **Club Directory:** Discover and join various campus clubs in one centralized place.
- **Discussion Feed:** Engage with the community through club-specific discussion boards.
- **Certificate Generation:** Automatically generate and distribute event certificates.
- **Realtime Updates:** Instant notifications and live updates powered by Supabase Realtime.

## 🛠️ Tech Stack

| Category            | Technology                                   |
| :------------------ | :------------------------------------------- |
| **Frontend**        | Vite, React, TypeScript, Tailwind CSS        |
| **Backend**         | Supabase (Postgres, Auth, Storage, Realtime) |
| **Package Manager** | Bun                                          |

## 🏗️ Architecture / Database

This project uses **Supabase (PostgreSQL)** as its backend database, with Supabase Auth and Row Level Security enabled for access control.

### Core Tables

#### profiles

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key; references Supabase Auth user |
| full_name | Text | User's full name |
| avatar_url | Text | Profile avatar image URL |
| college | Text | College or university name |
| bio | Text | Short biography |
| role | user_role | User role such as `student` or `club_admin` |
| created_at | Timestamp | Profile creation time |
| updated_at | Timestamp | Last profile update time |

#### clubs

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| name | Text | Club name |
| slug | Text | Unique URL-friendly club identifier |
| description | Text | Club description |
| banner_url | Text | Club banner image URL |
| logo_url | Text | Club logo image URL |
| created_by | UUID | Reference to the profile that created the club |
| created_at | Timestamp | Club creation time |
| updated_at | Timestamp | Last club update time |

#### club_members

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| club_id | UUID | References clubs.id |
| user_id | UUID | References profiles.id |
| role | member_role | Member or admin role |
| status | join_status | Membership status such as pending or approved |
| joined_at | Timestamp | Membership join time |

#### events

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| club_id | UUID | References clubs.id |
| title | Text | Event title |
| description | Text | Event description |
| banner_url | Text | Event banner image URL |
| event_date | Timestamp | Scheduled event date |
| location | Text | Event location |
| created_by | UUID | Reference to the profile that created the event |
| created_at | Timestamp | Event creation time |
| updated_at | Timestamp | Last event update time |

#### event_rsvps

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| event_id | UUID | References events.id |
| user_id | UUID | References profiles.id |
| checked_in | Boolean | Indicates whether the user checked in |
| rsvp_at | Timestamp | RSVP timestamp |

#### posts

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| club_id | UUID | References clubs.id |
| author_id | UUID | References profiles.id |
| content | Text | Post content |
| created_at | Timestamp | Post creation time |
| updated_at | Timestamp | Last post update time |

#### comments

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| post_id | UUID | References posts.id |
| author_id | UUID | References profiles.id |
| content | Text | Comment content |
| created_at | Timestamp | Comment creation time |
| updated_at | Timestamp | Last comment update time |

#### certificates

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| event_id | UUID | References events.id |
| user_id | UUID | References profiles.id |
| certificate_url | Text | Download URL for the certificate |
| issued_at | Timestamp | Certificate issue time |

### Relationships

- One profile can create many clubs, events, posts, and comments.
- One profile can receive many certificates.
- One club can have many members, events, and posts.
- One event can have many RSVPs and certificates.
- One post can have many comments.
- `club_members.club_id` → `clubs.id`
- `club_members.user_id` → `profiles.id`
- `events.club_id` → `clubs.id`
- `event_rsvps.event_id` → `events.id`
- `event_rsvps.user_id` → `profiles.id`
- `posts.club_id` → `clubs.id`
- `posts.author_id` → `profiles.id`
- `comments.post_id` → `posts.id`
- `comments.author_id` → `profiles.id`
- `certificates.event_id` → `events.id`
- `certificates.user_id` → `profiles.id`

### Entity Relationship Diagram (ERD)

```text
profiles ──< clubs
profiles ──< club_members >── clubs
events ──< event_rsvps >── profiles
clubs ──< posts >── comments >── profiles
events ──< certificates >── profiles
```

## 🚀 Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/krushit1307/CampusConnect.git
   cd CampusConnect
   ```
2. **Install dependencies:**
   ```bash
   bun install
   ```
3. **Set up environment variables:**
   ```bash
   cp .env.example .env.local
   ```
   Fill in your Supabase URL and Anon Key in `.env.local`.
4. **Run database migrations (if applicable):**
   ```bash
   supabase db push
   ```
5. **Start the development server:**
   ```bash
   bun run dev
   ```

## 📁 Project Structure

- `src/` — Contains all frontend React components, pages, hooks, and utilities.
- `supabase/` — Database migrations, seed data, and Edge Functions.
- `public/` — Static assets like images and fonts.

## 🤝 Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how to get started. This is an **ECSoC 2026** project, so we are actively looking for contributors. Check out issues labeled `good-first-issue` to begin!

## 🗺️ Roadmap

- **Phase 1:** Core web platform ✅
- **Phase 2:** Contributor feature build (In Progress)
- **Phase 3:** AI layer (Q4 2026) — AI event recommender, AI post summarizer, RAG chatbot via pgvector

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 👤 Maintainer

**Krushit Prajapati** - [GitHub Profile](https://github.com/krushit1307)
