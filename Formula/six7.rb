class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.6"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.6"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.6/six7-0.7.6-arm64-apple-darwin.tar.gz"
      sha256 "2b40a05ee84c8ccc9aa92ce850106f227cd546991699d974909994a2ac7e17d5"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
