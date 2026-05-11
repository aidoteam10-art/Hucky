import { render, screen } from '@testing-library/svelte';
import { describe, expect, it } from 'vitest';

import TournamentCard from './TournamentCard.svelte';

describe('TournamentCard', () => {
	it('renders tournament summary and details link', () => {
		render(TournamentCard, {
			props: {
				id: 'tournament-42',
				title: 'Hucky Spring Cup',
				description: 'A practical programming tournament',
				current_state: 'registration',
				start_date: '2026-05-15T10:00:00.000Z',
				rounds: 3,
				max_teams: 20,
				registered_teams: 7
			}
		});

		expect(screen.getByRole('heading', { name: 'Hucky Spring Cup' })).toBeInTheDocument();
		expect(screen.getByText('A practical programming tournament')).toBeInTheDocument();
		expect(screen.getByText(/7\/20/)).toBeInTheDocument();
		expect(screen.getByRole('link')).toHaveAttribute('href', '/tournaments/tournament-42');
	});
});
