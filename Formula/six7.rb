class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.49"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.49"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.49/six7-0.7.49-arm64-apple-darwin.tar.gz"
      sha256 "5c69a482426370f5a7a4b22e53b2670216d253729057b115db5b9b4515e0c060"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
