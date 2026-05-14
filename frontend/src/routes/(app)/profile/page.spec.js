import { fireEvent, render, screen } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';

vi.mock('$lib/certificatePdf', () => ({
	downloadCertificatePdf: vi.fn()
}));

import { downloadCertificatePdf } from '$lib/certificatePdf';
import Page from './+page.svelte';

const baseData = {
	profile: {
		full_name: 'Ada Lovelace',
		email: 'ada@example.com',
		role: 'participant',
		avatar_url: null
	},
	teams: [],
	invitations: [],
	juryAssignments: [],
	createdTournaments: [],
	certificates: []
};

describe('profile certificates', () => {
	it('does not render the certificates section without certificates', () => {
		render(Page, {
			props: {
				data: baseData,
				form: null
			}
		});

		expect(screen.queryByRole('heading', { name: 'Сертифікати' })).not.toBeInTheDocument();
	});

	it('renders certificates and downloads the selected certificate', async () => {
		const certificate = {
			id: 'certificate-1',
			userName: 'Ada Lovelace',
			teamName: 'Code Warriors',
			tournamentName: 'Hackathon Ukraine 2026',
			overallPlace: 2,
			issuedAt: '14.05.2026',
			rounds: [{ title: 'Раунд 1', place: 1 }]
		};

		render(Page, {
			props: {
				data: {
					...baseData,
					certificates: [certificate]
				},
				form: null
			}
		});

		expect(screen.getByRole('heading', { name: 'Сертифікати' })).toBeInTheDocument();
		expect(screen.getByText('Hackathon Ukraine 2026')).toBeInTheDocument();

		await fireEvent.click(screen.getByRole('button', { name: 'Завантажити сертифікат' }));

		expect(downloadCertificatePdf).toHaveBeenCalledWith(certificate);
	});
});
