import { fireEvent, render, screen } from '@testing-library/svelte';
import { tick } from 'svelte';
import { describe, expect, it } from 'vitest';

import Page from './+page.svelte';

const createAssignment = (overrides = {}) => ({
	id: 'assignment-1',
	status: 'pending',
	team: {
		id: 'team-1',
		name: 'Team Alpha'
	},
	submission: {
		github_url: 'https://github.com/example/project',
		video_demo_url: 'https://video.example/demo',
		live_demo_url: null,
		description: 'Submission description'
	},
	criteria: [
		{
			id: 'criterion-1',
			code: 'backend',
			name: 'Backend',
			description: 'Backend quality',
			max_score: 100,
			score: 0
		},
		{
			id: 'criterion-2',
			code: 'frontend',
			name: 'Frontend',
			description: 'Frontend quality',
			max_score: 100,
			score: 20
		}
	],
	comment: '',
	...overrides
});

function renderPage(assignment = createAssignment()) {
	return render(Page, {
		props: {
			data: { assignment },
			form: null
		}
	});
}

describe('jury grading page', () => {
	it('updates the criterion score, hidden input, and total score from the slider', async () => {
		const { container } = renderPage();

		await fireEvent.input(screen.getByRole('slider', { name: 'Score slider for Backend' }), {
			target: { value: '55' }
		});
		await tick();

		expect(screen.getByRole('spinbutton', { name: 'Score for Backend' })).toHaveValue(55);
		expect(container.querySelectorAll('input[name="score"]')[0]).toHaveValue('55');
		expect(screen.getByTestId('total-score')).toHaveTextContent('75');
	});

	it('updates the slider-backed state and total score from manual score input', async () => {
		const { container } = renderPage();

		await fireEvent.input(screen.getByRole('spinbutton', { name: 'Score for Frontend' }), {
			target: { value: '40' }
		});
		await tick();

		expect(screen.getByRole('slider', { name: 'Score slider for Frontend' })).toHaveValue('40');
		expect(container.querySelectorAll('input[name="score"]')[1]).toHaveValue('40');
		expect(screen.getByTestId('total-score')).toHaveTextContent('40');
	});

	it('keeps completed assignments disabled and read-only', () => {
		renderPage(
			createAssignment({
				status: 'completed',
				comment: 'Already graded',
				criteria: [
					{
						id: 'criterion-1',
						code: 'backend',
						name: 'Backend',
						description: 'Backend quality',
						max_score: 100,
						score: 80
					}
				]
			})
		);

		expect(screen.getByRole('spinbutton', { name: 'Score for Backend' })).toBeDisabled();
		expect(screen.getByRole('slider', { name: 'Score slider for Backend' })).toBeDisabled();
		expect(screen.getByPlaceholderText('Додатковий відгук...')).toHaveAttribute('readonly');
		expect(screen.queryByRole('button', { name: 'Підтвердити оцінювання' })).not.toBeInTheDocument();
		expect(screen.getByText('Оцінено')).toBeInTheDocument();
	});
});
