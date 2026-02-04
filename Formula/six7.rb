class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.9"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.9"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.9/six7-0.7.9-arm64-apple-darwin.tar.gz"
      sha256 "6666cbb90710541320c7a1b055246203cdb84eb8d0e1ac59b76087e6c3eea456"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
