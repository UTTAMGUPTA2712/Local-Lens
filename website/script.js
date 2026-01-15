document.addEventListener('DOMContentLoaded', async () => {
    const versionContainer = document.getElementById('version-container');
    const prevContainer = document.getElementById('previous-versions-container');
    const prevList = document.getElementById('previous-versions-list');

    try {
        const versions = [
            {
                "version": "v0.1.2",
                "release_date": "2024-01-15",
                "description": "Initial release of Local Lens with object detection, OCR, and metadata embedding.",
                "downloads": {
                    "linux": "https://github.com/UTTAMGUPTA2712/Local-Lens/actions/runs/20952092371/artifacts/5109948195",
                    "windows": "https://github.com/UTTAMGUPTA2712/Local-Lens/actions/runs/20952092371/artifacts/5109972264",
                    "mac": "https://github.com/UTTAMGUPTA2712/Local-Lens/actions/runs/20952092371/artifacts/5109951540"
                }
            },
            {
                "version": "v0.1.1",
                "release_date": "2023-12-20",
                "description": "Beta release. Basic object detection and tagging.",
                "downloads": {
                    "linux": "#",
                    "windows": "#",
                    "mac": "#"
                }
            },
            {
                "version": "v0.1.0",
                "release_date": "2023-12-20",
                "description": "Beta release. Basic object detection and tagging.",
                "downloads": {
                    "linux": "https://github.com/UTTAMGUPTA2712/Local-Lens/actions/runs/20948983561/artifacts/5108816967",
                    "windows": "https://github.com/UTTAMGUPTA2712/Local-Lens/actions/runs/20948983561/artifacts/5108832761",
                    "mac": "https://github.com/UTTAMGUPTA2712/Local-Lens/actions/runs/20948983561/artifacts/5108841929"
                }
            },
        ];

        if (!versions || versions.length === 0) {
            versionContainer.innerHTML = '<p>No version information available.</p>';
            return;
        }

        const latest = versions[0];
        renderDownloadSection(latest);

        // Render previous versions if any
        if (versions.length > 1) {
            const olderVersions = versions.slice(1);
            renderPreviousVersions(olderVersions);
            prevContainer.style.display = 'block';
        }

    } catch (error) {
        console.error(error);
        versionContainer.innerHTML = '<p style="color: #ff7675;">Error loading download links. Please check back later.</p>';
    }

    function renderDownloadSection(version) {
        const html = `
            <div class="version-info">
                <h2>Latest Release: <span style="color: var(--secondary-color)">${version.version}</span></h2>
                <span class="version-date">Released: ${version.release_date}</span>
                <p style="color: var(--text-gray); margin-top: 10px;">${version.description}</p>
            </div>
            
            <div class="os-grid">
                <a href="${version.downloads.linux}" class="download-card">
                    <div class="os-icon">🐧</div>
                    <span class="os-name">Linux</span>
                    <small style="color: var(--text-gray)">tar.gz</small>
                </a>
                
                <a href="${version.downloads.windows}" class="download-card">
                    <div class="os-icon">🪟</div>
                    <span class="os-name">Windows</span>
                    <small style="color: var(--text-gray)">.zip</small>
                </a>
                
                <a href="${version.downloads.mac}" class="download-card">
                    <div class="os-icon">🍎</div>
                    <span class="os-name">macOS</span>
                    <small style="color: var(--text-gray)">tar.gz (Intel/Silicon)</small>
                </a>
            </div>
        `;

        versionContainer.innerHTML = html;
    }

    function renderPreviousVersions(versions) {
        const html = versions.map(v => {
            const linuxBtn = isValidUrl(v.downloads.linux) ? `<a href="${v.downloads.linux}" class="mini-btn">Linux</a>` : '';
            const winBtn = isValidUrl(v.downloads.windows) ? `<a href="${v.downloads.windows}" class="mini-btn">Win</a>` : '';
            const macBtn = isValidUrl(v.downloads.mac) ? `<a href="${v.downloads.mac}" class="mini-btn">Mac</a>` : '';

            // Only render buttons group if at least one link exists
            const buttonsHtml = (linuxBtn || winBtn || macBtn)
                ? `<div class="prev-downloads">${linuxBtn}${winBtn}${macBtn}</div>`
                : `<div class="prev-downloads"><span style="color: var(--text-gray); font-size: 0.85rem;">No downloads available</span></div>`;

            return `
            <div class="previous-version-item">
                <div class="prev-version-info">
                    <span class="prev-version-name">${v.version}</span>
                    <span class="prev-version-date">${v.release_date}</span>
                    <p class="prev-version-desc">${v.description}</p>
                </div>
                ${buttonsHtml}
            </div>
        `}).join('');

        prevList.innerHTML = html;
    }

    function isValidUrl(url) {
        return url && url.length > 1 && url !== '#';
    }
});
