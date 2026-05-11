import { render } from '@testing-library/svelte';
import { describe, expect, it } from 'vitest';

import StateTag from './StateTag.svelte';

describe('StateTag', () => {
	it('uses the running status style for running tournaments', () => {
		const { container } = render(StateTag, {
			props: {
				variant: 'running'
			}
		});

		const tag = container.querySelector('span');
		expect(tag).toBeInTheDocument();
		expect(tag).toHaveClass('text-[#F97316]');
	});

	it('falls back to registration styling for an unknown status', () => {
		const { container } = render(StateTag, {
			props: {
				variant: 'unexpected',
				class: 'text-xs'
			}
		});

		const tag = container.querySelector('span');
		expect(tag).toHaveClass('text-[#CCFF00]');
		expect(tag).toHaveClass('text-xs');
	});
});
