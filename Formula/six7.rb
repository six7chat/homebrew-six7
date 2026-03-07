class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.64"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.64"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.64/six7-0.7.64-arm64-apple-darwin.tar.gz"
      sha256 "32746f8b3aaccf96b1288de0a4c93926dc37e606bd71f236ae633fa8305268f7"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
