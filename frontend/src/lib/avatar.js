import { writable } from 'svelte/store';

export const DEFAULT_AVATAR = '/icons/avatar.svg';

export const avatarSrc = writable(DEFAULT_AVATAR);

export function setAvatar(src) {
	avatarSrc.set(src || DEFAULT_AVATAR);
}

export function isDefaultAvatar(src) {
	return !src || src === DEFAULT_AVATAR;
}
