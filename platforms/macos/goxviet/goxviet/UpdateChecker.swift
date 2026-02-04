//
//  UpdateChecker.swift
//  GoxViet
//
//  Dedicated class for checking updates via GitHub API
//

import Foundation

// MARK: - Update Info

struct UpdateInfo: Equatable {
    let version: String
    let downloadURL: URL
    let releaseNotes: String
    let publishedAt: Date?
}

// MARK: - Update Check Result

enum UpdateCheckResult {
    case available(UpdateInfo)
    case upToDate
    case error(String)
}

// MARK: - Update Checker

class UpdateChecker {
    static let shared = UpdateChecker()

    private let githubAPIURL = "https://api.github.com/repos/nihmtaho/goxviet-ime/releases/latest"

    private init() {}

    /// Check for updates asynchronously
    func checkForUpdates(completion: @escaping (UpdateCheckResult) -> Void) {
        guard let url = URL(string: githubAPIURL) else {
            completion(.error("Invalid API URL"))
            return
        }

        var request = URLRequest(url: url)
        request.setValue("application/vnd.github.v3+json", forHTTPHeaderField: "Accept")
        request.setValue("GoxViet-Update-Agent", forHTTPHeaderField: "User-Agent")
        request.timeoutInterval = 15
        request.cachePolicy = .reloadIgnoringLocalCacheData

        let task = URLSession.shared.dataTask(with: request) { [weak self] data, response, error in
            if let error = error {
                DispatchQueue.main.async {
                    completion(.error("Network error: \(error.localizedDescription)"))
                }
                return
            }

            guard let httpResponse = response as? HTTPURLResponse else {
                DispatchQueue.main.async {
                    completion(.error("Invalid response"))
                }
                return
            }

            guard httpResponse.statusCode == 200 else {
                DispatchQueue.main.async {
                    completion(.error("Server error: \(httpResponse.statusCode)"))
                }
                return
            }

            guard let data = data else {
                DispatchQueue.main.async {
                    completion(.error("No data received"))
                }
                return
            }

            self?.parseResponse(data: data, completion: completion)
        }

        task.resume()
    }

    private func parseResponse(data: Data, completion: @escaping (UpdateCheckResult) -> Void) {
        do {
            let release = try JSONDecoder().decode(ReleaseResponse.self, from: data)
            
            guard let version = normalizeVersion(release.tagName),
                  let currentVersion = normalizeVersion(Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String) else {
                DispatchQueue.main.async { completion(.error("Could not determine versions")) }
                return
            }
            
            if isNewerVersion(version, than: currentVersion) {
                guard let downloadURL = release.preferredDownloadURL else {
                     DispatchQueue.main.async { completion(.error("No DMG found in release")) }
                     return
                }
                
                let info = UpdateInfo(
                    version: version,
                    downloadURL: downloadURL,
                    releaseNotes: release.body,
                    publishedAt: ISO8601DateFormatter().date(from: release.publishedAt)
                )
                DispatchQueue.main.async { completion(.available(info)) }
            } else {
                DispatchQueue.main.async { completion(.upToDate) }
            }

        } catch {
            DispatchQueue.main.async {
                completion(.error("JSON parse error: \(error.localizedDescription)"))
            }
        }
    }
    
    // MARK: - Version Comparison Helpers
    
    private func normalizeVersion(_ version: String?) -> String? {
        guard let version = version else { return nil }
        return version.trimmingCharacters(in: CharacterSet(charactersIn: "vV"))
    }

    private func isNewerVersion(_ latest: String, than current: String) -> Bool {
        let latestParts = latest.split(separator: ".").compactMap { Int($0) }
        let currentParts = current.split(separator: ".").compactMap { Int($0) }

        let maxCount = max(latestParts.count, currentParts.count)
        for idx in 0..<maxCount {
            let latestValue = idx < latestParts.count ? latestParts[idx] : 0
            let currentValue = idx < currentParts.count ? currentParts[idx] : 0
            if latestValue > currentValue { return true }
            if latestValue < currentValue { return false }
        }
        return false
    }
}

// MARK: - Release Models

private struct ReleaseResponse: Decodable {
    let tagName: String
    let htmlURL: String
    let body: String
    let publishedAt: String
    let assets: [ReleaseAsset]

    enum CodingKeys: String, CodingKey {
        case tagName = "tag_name"
        case htmlURL = "html_url"
        case body
        case publishedAt = "published_at"
        case assets
    }

    var preferredDownloadURL: URL? {
        if let dmgAsset = assets.first(where: { $0.name.lowercased().hasSuffix(".dmg") }) {
            return URL(string: dmgAsset.browserDownloadURL)
        }
        return URL(string: htmlURL)
    }
}

private struct ReleaseAsset: Decodable {
    let name: String
    let browserDownloadURL: String

    enum CodingKeys: String, CodingKey {
        case name
        case browserDownloadURL = "browser_download_url"
    }
}
