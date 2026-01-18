class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.7"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.7"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.7/six7-0.7.7-arm64-apple-darwin.tar.gz"
      sha256 "4baefd93b10d890e7d61303c1ce6c14ba234e799c4be3871c807fa97aae477cd"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
