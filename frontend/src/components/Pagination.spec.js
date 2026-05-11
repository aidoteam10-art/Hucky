import { fireEvent, render, screen } from '@testing-library/svelte';
import { describe, expect, it } from 'vitest';

import Pagination from './Pagination.svelte';

describe('Pagination', () => {
	it('disables the previous button on the first page', () => {
		render(Pagination, {
			props: {
				currentPage: 1,
				totalPages: 3
			}
		});

		const buttons = screen.getAllByRole('button');
		expect(buttons[0]).toBeDisabled();
		expect(buttons.at(-1)).not.toBeDisabled();
		expect(screen.getByRole('button', { name: '1' })).toHaveClass('bg-[#CCFF00]');
	});

	it('moves to the selected page and disables next on the last page', async () => {
		render(Pagination, {
			props: {
				currentPage: 1,
				totalPages: 2
			}
		});

		await fireEvent.click(screen.getByRole('button', { name: '2' }));

		const buttons = screen.getAllByRole('button');
		expect(screen.getByRole('button', { name: '2' })).toHaveClass('bg-[#CCFF00]');
		expect(buttons.at(-1)).toBeDisabled();
	});
});
