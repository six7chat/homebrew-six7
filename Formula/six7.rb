class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.12"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.12"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.12/six7-0.7.12-arm64-apple-darwin.tar.gz"
      sha256 "46ff2543f7cfdfc5659385133c448a9dad4ca7ac422d8799c2e1b8f384ee7040"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
