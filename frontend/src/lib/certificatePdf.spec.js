import { describe, expect, it } from 'vitest';

import { certificateRoundPages, formatPlace } from './certificatePdf';

describe('certificatePdf helpers', () => {
	it('renders missing places as not submitted', () => {
		expect(formatPlace(null)).toBe('Не подано');
		expect(formatPlace(undefined)).toBe('Не подано');
		expect(formatPlace(2)).toBe('2 місце');
	});

	it('adds extra pages only when the first page cannot fit all rounds', () => {
		expect(certificateRoundPages([{ title: 'Раунд 1', place: 1 }])).toEqual([]);

		const rounds = Array.from({ length: 21 }, (_, index) => ({
			title: `Раунд ${index + 1}`,
			place: index + 1
		}));

		expect(certificateRoundPages(rounds)).toHaveLength(2);
		expect(certificateRoundPages(rounds)[0]).toHaveLength(16);
		expect(certificateRoundPages(rounds)[1]).toHaveLength(5);
	});
});
