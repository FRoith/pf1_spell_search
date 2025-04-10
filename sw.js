var cacheName = 'pf1-spell-search-pwa';
var filesToCache = [
  './',
  './index.html',
  './pf1_spell_search-10769b3f.js',
  './pf1_spell_search_bg-e1555cea.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});
