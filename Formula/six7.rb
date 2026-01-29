class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.10"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.10"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.10/six7-0.7.10-arm64-apple-darwin.tar.gz"
      sha256 "aa073cea38fb0c43bf027af1660b9bf5e3b1219dbb7b88e0e9301cc9f8120b46"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
