import { fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest, getAuthToken } from '$lib/server/api';

export const load = async ({ cookies }) => {
	return {
		isAuthenticated: Boolean(getAuthToken(cookies))
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
			return fail(400, { message: payload.error, values: Object.fromEntries(formData) });
		}

		let created;

		try {
			created = await apiRequest(fetch, '/api/tournaments', {
				method: 'POST',
				token,
				body: payload.data
			});
		} catch (error) {
			if (error instanceof ApiError) {
				return fail(error.status, {
					message: error.message,
					values: Object.fromEntries(formData)
				});
			}

			return fail(500, {
				message: 'Backend недоступний',
				values: Object.fromEntries(formData)
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
	const roundTitle = readString(formData, 'round_title');
	const taskDescription = readString(formData, 'task_description');
	const technologyRequirements = readString(formData, 'technology_requirements');
	const roundStartsAt = readDate(formData, 'round_starts_at');
	const deadlineAt = readDate(formData, 'deadline_at');
	const mustHave = formData
		.getAll('must_have')
		.map((value) => String(value).trim())
		.filter(Boolean);

    console.dir({
        title, description, rules, registration_starts_at, registration_ends_at, starts_at, max_teams, roundTitle, taskDescription, technologyRequirements, roundStartsAt, deadlineAt, mustHave
    });

	if (!title || !description || !rules || !registration_starts_at || !registration_ends_at || !starts_at) {
		return { error: 'Заповніть деталі турніру та дати' };
	}

	if (!roundTitle || !taskDescription || !roundStartsAt || !deadlineAt) {
		return { error: 'Заповніть дані першого раунду' };
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
				title: roundTitle,
				task_description: taskDescription,
				technology_requirements: technologyRequirements || null,
				must_have: mustHave,
				starts_at: roundStartsAt,
				deadline_at: deadlineAt
			}
		}
	};
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
