import { browser } from '$app/environment';
import { writable } from 'svelte/store';

export const DEFAULT_AVATAR = '/icons/avatar.svg';
export const AVATAR_STORAGE_KEY = 'hucky.profile.avatar';

function initialAvatar() {
	if (!browser) return DEFAULT_AVATAR;
	return localStorage.getItem(AVATAR_STORAGE_KEY) || DEFAULT_AVATAR;
}

export const avatarSrc = writable(initialAvatar());

export function saveAvatar(dataUrl) {
	if (!browser) return;
	localStorage.setItem(AVATAR_STORAGE_KEY, dataUrl);
	avatarSrc.set(dataUrl);
}

export function resetAvatar() {
	if (browser) {
		localStorage.removeItem(AVATAR_STORAGE_KEY);
	}
	avatarSrc.set(DEFAULT_AVATAR);
}

export function isDefaultAvatar(src) {
	return !src || src === DEFAULT_AVATAR;
}
