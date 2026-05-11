import { render, screen } from '@testing-library/svelte';
import { describe, expect, it } from 'vitest';

import InputField from './InputField.svelte';

describe('InputField', () => {
	it('renders a named input with its current value', () => {
		render(InputField, {
			props: {
				name: 'email',
				header: 'Email',
				type: 'email',
				value: 'student@example.com',
				required: true
			}
		});

		const input = screen.getByDisplayValue('student@example.com');
		expect(input).toHaveAttribute('name', 'email');
		expect(input).toHaveAttribute('type', 'email');
		expect(input).toBeRequired();
	});

	it('shows validation errors beside the input', () => {
		render(InputField, {
			props: {
				name: 'team',
				header: 'Team',
				error: 'Team name is required'
			}
		});

		expect(screen.getByText('Team name is required')).toBeInTheDocument();
		expect(screen.getByPlaceholderText('Example text')).toHaveClass('border-red-500');
	});
});
