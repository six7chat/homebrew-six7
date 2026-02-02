class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.11"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.11"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.11/six7-0.7.11-arm64-apple-darwin.tar.gz"
      sha256 "5d49f8c9293a87baee51a3cd69b64efd3b2e8025776b85ed877b285c1108b88a"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
