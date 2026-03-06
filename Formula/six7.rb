class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.58"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.58"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.58/six7-0.7.58-arm64-apple-darwin.tar.gz"
      sha256 "4ec9a05e25663dd0f4a30b85a1d94b90be457135b566838ef87de3d1323efbbf"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
