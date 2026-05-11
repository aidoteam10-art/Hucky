import { error as kitError, fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ cookies, fetch }) => {
	const token = getAuthToken(cookies);
	if (!token) {
		throw redirect(303, '/login');
	}

	const profile = await apiRequest(fetch, '/api/users/me', { token });
	if (profile.role !== 'organiser') {
		throw kitError(403, 'Only organisers can create tournaments');
	}

	return {
		isAuthenticated: true,
		canCreateTournament: true
	};
};

export const actions = {
	default: async ({ request, cookies, fetch }) => {
		const token = getAuthToken(cookies);

		if (!token) {
			return fail(401, { message: 'Увійдіть, щоб створити турнір' });
		}

		const formData = await request.formData();
		const payload = buildTournamentPayload(formData);

		if (payload.error) {
			return fail(400, { message: payload.error, values: payload.values });
		}

		let created;

		try {
			created = await apiRequest(fetch, '/api/tournaments', {
				method: 'POST',
				token,
				body: payload.data
			});

			for (const round of payload.additionalRounds) {
				await apiRequest(fetch, `/api/tournaments/${created.id}/rounds`, {
					method: 'POST',
					token,
					body: round
				});
			}
		} catch (error) {
			if (error instanceof ApiError) {
				return fail(error.status, {
					message: error.message,
					values: payload.values
				});
			}

			return fail(500, {
				message: 'Backend недоступний',
				values: payload.values
			});
		}

		throw redirect(303, `/tournaments/${created.id}`);
	}
};

function buildTournamentPayload(formData) {
	const title = readString(formData, 'title');
	const description = readString(formData, 'description');
	const rules = readString(formData, 'rules') || description;
	const registration_starts_at = readDate(formData, 'registration_starts_at');
	const registration_ends_at = readDate(formData, 'registration_ends_at');
	const starts_at = readDate(formData, 'starts_at');
	const max_teams = readOptionalNumber(formData, 'max_teams');
	const rounds = readRounds(formData);
	const values = {
		title,
		description,
		rules,
		registration_starts_at: readString(formData, 'registration_starts_at'),
		registration_ends_at: readString(formData, 'registration_ends_at'),
		starts_at: readString(formData, 'starts_at'),
		max_teams: readString(formData, 'max_teams'),
		rounds: rounds.map((round) => ({
			title: round.title,
			taskDescription: round.task_description,
			technologyRequirements: round.technology_requirements || '',
			startsAt: round.raw_starts_at,
			deadlineAt: round.raw_deadline_at,
			requirements: round.must_have.length ? round.must_have : ['']
		}))
	};

	if (!title || !description || !rules || !registration_starts_at || !registration_ends_at || !starts_at) {
		return { error: 'Заповніть деталі турніру та дати', values };
	}

	if (rounds.length === 0) {
		return { error: 'Додайте хоча б один раунд', values };
	}

	const incompleteRoundIndex = rounds.findIndex(
		(round) => !round.title || !round.task_description || !round.starts_at || !round.deadline_at
	);

	if (incompleteRoundIndex !== -1) {
		return {
			error: `Заповніть дані раунду ${incompleteRoundIndex + 1}`,
			values
		};
	}

	return {
		data: {
			title,
			description,
			rules,
			registration_starts_at,
			registration_ends_at,
			starts_at,
			max_teams,
			first_round: {
				title: rounds[0].title,
				task_description: rounds[0].task_description,
				technology_requirements: rounds[0].technology_requirements || null,
				must_have: rounds[0].must_have,
				starts_at: rounds[0].starts_at,
				deadline_at: rounds[0].deadline_at
			}
		},
		additionalRounds: rounds.slice(1).map((round) => ({
			title: round.title,
			task_description: round.task_description,
			technology_requirements: round.technology_requirements || null,
			must_have: round.must_have,
			starts_at: round.starts_at,
			deadline_at: round.deadline_at
		})),
		values
	};
}

function readRounds(formData) {
	const explicitCount = readOptionalNumber(formData, 'round_count');
	const roundCount = explicitCount && explicitCount > 0 ? explicitCount : 1;
	const rounds = [];

	for (let index = 0; index < roundCount; index += 1) {
		const title = readString(formData, `round_${index}_title`) || readString(formData, 'round_title');
		const task_description =
			readString(formData, `round_${index}_task_description`) || readString(formData, 'task_description');
		const technology_requirements =
			readString(formData, `round_${index}_technology_requirements`) ||
			readString(formData, 'technology_requirements');
		const raw_starts_at =
			readString(formData, `round_${index}_starts_at`) || readString(formData, 'round_starts_at');
		const raw_deadline_at =
			readString(formData, `round_${index}_deadline_at`) || readString(formData, 'deadline_at');
		const starts_at = readDate(formData, `round_${index}_starts_at`) || readDate(formData, 'round_starts_at');
		const deadline_at = readDate(formData, `round_${index}_deadline_at`) || readDate(formData, 'deadline_at');
		const must_have = formData
			.getAll(`round_${index}_must_have`)
			.concat(index === 0 ? formData.getAll('must_have') : [])
			.map((value) => String(value).trim())
			.filter(Boolean);

		rounds.push({
			title,
			task_description,
			technology_requirements,
			raw_starts_at,
			raw_deadline_at,
			starts_at,
			deadline_at,
			must_have
		});
	}

	return rounds;
}

function readString(formData, name) {
	return String(formData.get(name) || '').trim();
}

function readDate(formData, name) {
	const value = readString(formData, name);
	if (!value) return null;

	const datetimeLocalMatch = value.match(
		/^(\d{4}-\d{2}-\d{2})[T ](\d{2}:\d{2})(?::(\d{2})(\.\d{1,3})?)?$/
	);
	if (datetimeLocalMatch) {
		const [, datePart, timePart, seconds = '00', fraction = ''] = datetimeLocalMatch;
		const normalized = `${datePart}T${timePart}:${seconds}${fraction || ''}`;
		const date = new Date(normalized);
		if (!Number.isNaN(date.getTime())) return date.toISOString();
	}

	const date = new Date(value);
	if (Number.isNaN(date.getTime())) return null;
	return date.toISOString();
}

function readOptionalNumber(formData, name) {
	const value = readString(formData, name);
	if (!value) return null;
	const number = Number(value);
	return Number.isFinite(number) ? number : null;
}
