class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.42"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.42"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.42/six7-0.7.42-arm64-apple-darwin.tar.gz"
      sha256 "6f6e8e8e05eccf9f1eeb34455519fb0acfab713b75fb3e600f70014e7b5968ad"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
