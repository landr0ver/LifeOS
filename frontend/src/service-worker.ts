/// <reference types="@sveltejs/kit" />
/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />

import { build, files, version } from '$service-worker';

const sw = self as unknown as ServiceWorkerGlobalScope;

const CACHE_NAME = `lifeos-cache-${version}`;
const ASSETS = [...build, ...files];

sw.addEventListener('install', (event) => {
	event.waitUntil(
		caches
			.open(CACHE_NAME)
			.then((cache) => cache.addAll(ASSETS))
			.then(() => sw.skipWaiting())
	);
});

sw.addEventListener('activate', (event) => {
	event.waitUntil(
		caches.keys().then(async (keys) => {
			for (const key of keys) {
				if (key !== CACHE_NAME) {
					await caches.delete(key);
				}
			}
			await sw.clients.claim();
		})
	);
});

sw.addEventListener('fetch', (event) => {
	if (event.request.method !== 'GET') return;

	const url = new URL(event.request.url);

	// Skip API requests for cache (network-first for API)
	if (url.pathname.startsWith('/api')) {
		event.respondWith(
			fetch(event.request).catch(() =>
				caches.match(event.request).then((r) => r || new Response('Offline', { status: 503 }))
			)
		);
		return;
	}

	// Cache-first for static assets
	if (ASSETS.includes(url.pathname)) {
		event.respondWith(
			caches.match(event.request).then((r) => r || fetch(event.request))
		);
		return;
	}

	// Network-first for pages
	event.respondWith(
		fetch(event.request)
			.then((response) => {
				const clone = response.clone();
				caches.open(CACHE_NAME).then((cache) => cache.put(event.request, clone));
				return response;
			})
			.catch(() =>
				caches.match(event.request).then((r) => r || caches.match('/'))
			)
			.then((r) => r || new Response('Offline', { status: 503 }))
	);
});
