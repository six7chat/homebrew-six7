class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.50"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.50"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.50/six7-0.7.50-arm64-apple-darwin.tar.gz"
      sha256 "781bf4b4882757f525339e9bf458c01d6888f12d604d568a404d39ccb922beb7"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
