class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.10"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.10"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.10/six7-0.7.10-arm64-apple-darwin.tar.gz"
      sha256 "e408e2f5ce721c69c8176ff300d1b71305327d429903a7b05da0157447da1b28"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
